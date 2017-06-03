use diesel;
use diesel::prelude::*;

use data::types::*;
use data::researches::get_research_list;
use data::helper::{get_research_dependency_strings, dependencies_fulfilled};
use helpers::db::DB;
use responses::{APIResponse, bad_request, created, ok};

use models::user::User;
use models::research::{Research, NewResearch};
use models::resource::Resource;
use models::queue::{QueueEntry, NewQueueEntry};

use schema::researches;
use schema::researches::dsl as research_dsl;
use schema::queue_entries::dsl as queue_entry_dsl;


/// The user needs to be logged in to access this route!
///
/// This route returns the list of all researches and their levels/costs,
/// as well as the current level of the research for the pod of the current user.
#[get("/pod")]
pub fn get_researches(current_user: User, db: DB) -> APIResponse {

    let mut research_list = get_research_list();

    // Ger current pod and pod researches
    let pod = current_user.get_pod(&db);
    
    let researches = pod.get_researches(&db);
    for research in researches {
        let type_result = ResearchTypes::from_string(&research.name);
        if type_result.is_err() {
            return bad_request()
                       .message(format!("Found research {}, but no matching ResearchType!",
                                        research.name)
                                        .as_str());
        }
        let research_type = type_result.unwrap();
        let list_result = research_list.get_mut(&research_type);
        if list_result.is_none() {
            return bad_request().message(format!("Found type {}, but no matching entry in our research list!", research_type).as_str());
        }

        let mut list_entry = list_result.unwrap();
        list_entry.current_level = research.level;
    }

    ok().message("Research data.").data(json!(&research_list))
}

/// Add a new research to the queue of the pod
/// This endpoint:
/// - Checks if dependencies for research are fulfilled
/// - Checks if there are enough resources
/// - Removes resources from db
#[post("/pod/<research_name>")]
pub fn start_research(research_name: &str,
                          current_user: User,
                          db: DB)
                          -> APIResponse {

    // Check if the given research name maps to a research type.
    // Early return if we don't know this research name
    let research_result = ResearchTypes::from_str(research_name);
    if research_result.is_err() {
        return bad_request().message(format!("No such research type `{}`", research_name).as_str());
    }
    let research_type =  research_result.unwrap();

    // Get and set some variables we need for querying and dependency checking.
    let dependency_strings = get_research_dependency_strings(&research_type);
    let mut research_level: i32;
    let research_list = get_research_list();

    let (pod, queue) = current_user.get_pod_and_queue(&db);

    let mut research_result = pod.get_research(research_type.to_string(), &db);

    let research: Research;
    if research_result.is_ok() {
        research = research_result.unwrap();
        research_level = research.level + 1;
    }
    else {
        let dependencies = research_dsl::researches
            .filter(research_dsl::name.eq_any(dependency_strings))
            .get_results::<Research>(&*db);

        let fulfilled = dependencies_fulfilled(&research_type,
                                               dependencies,
                                               &research_list);
        if !fulfilled {
            return bad_request().message("Dependencies not fulfilled.");
        }
        // Create a new module in the
        let new_research = NewResearch {
            name: research_type.to_string(),
            pod_id: Some(pod.id),
            base_id: None,
        };

        research_result = diesel::insert(&new_research)
            .into(researches::table)
            .get_result::<Research>(&*db);
        
        research = research_result.expect("Failed to create research.");

        research_level = 1;
    }

    // Check if there already are existing queue entries for this research.
    // In case there are, we increase the level by the amount of existing entries.
    let existing_entries: i64 = queue_entry_dsl::queue_entries
        .count()
        .filter(queue_entry_dsl::queue_id.eq(queue.id))
        .filter(queue_entry_dsl::research_name.eq(research_type.to_string()))
        .get_result(&*db)
        .unwrap_or(0);

    let pod_resources = pod.get_resources(&db);
    research_level += existing_entries as i32;

    let all_levels = &research_list
                          .get(&research_type)
                          .as_ref()
                          .expect("No research in yml for this type.")
                          .levels;

    if research_level > all_levels.len() as i32 {
        return bad_request().message("Already at max level.");
    }

    let level_index: usize = (research_level-1) as usize;
    let costs = &all_levels[level_index].resources;

    if costs.is_some() && !Resource::check_resources(costs, pod_resources, &db) {
        return bad_request().message("Insufficient resources.");
    }

    // Create a new queue entry with the given research type.
    let new_queue_entry = NewQueueEntry {
        queue_id: queue.id.clone(),
        research_id: Some(research.id.clone()),
        research_name: Some(research_name.to_string().clone()),
        module_name: None,
        module_id: None,
        level: research_level,
        duration: all_levels[level_index].time,
    };
    queue.add_entry(new_queue_entry, &db);

    created().message("Queue entry added.")
}


/// Remove research from queue
#[delete("/pod/<research_name>")]
pub fn stop_research(research_name: &str, current_user: User, db: DB) -> APIResponse {

    // Check if there is a research for this research_name
    let research_type_result = ResearchTypes::from_str(research_name);
    // Early return if we don't know this research name
    if research_type_result.is_err() {
        return bad_request().message(format!("No such research type `{}`", research_name).as_str());
    }
    let research_type = research_type_result.unwrap();

    // Get user pod and pod queue
    let (pod, queue) = current_user.get_pod_and_queue(&db);

    // Check if there exists a queue entry for this research and this pod.
    // Early return if this isn't the case.
    let research_entry_result = queue_entry_dsl::queue_entries
        .filter(queue_entry_dsl::queue_id.eq(queue.id))
        .filter(queue_entry_dsl::research_name.eq(research_type.to_string()))
        .order(queue_entry_dsl::level.desc())
        .get_result::<QueueEntry>(&*db);
    if research_entry_result.is_ok() {
        return bad_request().message("Can't delete. There is no queue entry for this research.");
    }
    let research_entry = research_entry_result.unwrap();

    // Get all needed info for resource manipulation
    let research_list = get_research_list();

    let pod_resources = pod.get_resources(&db);

    // Add resources from research to pod resources
    let all_levels = &research_list
                          .get(&research_type)
                          .unwrap()
                          .levels;
    let costs_result = &all_levels[research_entry.level as usize].resources;

    if let Some(ref costs) = *costs_result {
        Resource::update_resources(costs, pod_resources, false, &db);
    }

    // Remove queue_entry from database
    diesel::delete(queue_entry_dsl::queue_entries
                       .filter(queue_entry_dsl::id.eq(research_entry.id)))
            .execute(&*db)
            .expect("Failed to remove queue_entry.");

    ok().message("Resource removed.")
}
