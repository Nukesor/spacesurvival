use diesel;
use diesel::prelude::*;
use rocket_contrib::{JSON, SerdeError};

use data::types::*;
use data::researches::get_research_list;
use data::helper::{get_research_dependency_strings, dependencies_fulfilled};

use helpers::db::DB;
use responses::{APIResponse, bad_request, created, ok};

use models::pod::Pod;
use models::user::User;
use models::research::Research;
use models::resource::Resource;

use schema::queue_entries;
use schema::pods::dsl as pods_dsl;
use schema::queues::dsl as queues_dsl;
use schema::resources::dsl as resources_dsl;
use schema::researches::dsl as research_dsl;
use schema::queue_entries::dsl as queue_entries_dsl;

use models::queue::{QueueEntry, Queue, NewQueueEntry};
use validation::queue::QueueAddResearchSerializer;


/// The user needs to be logged in to access this route!
///
/// This route returns the list of all researches and their levels/costs,
/// as well as the current level of the research for the pod of the current user.
#[get("/pod")]
pub fn pod_research(current_user: User, db: DB) -> APIResponse {

    let mut research_list = get_research_list();
    // Create changed pod model and push it to the DB

    let pod = pods_dsl::pods
        .filter(pods_dsl::user_id.eq(current_user.id))
        .get_result::<Pod>(&*db)
        .expect("Failed to get user pod.");

    let pod_result = research_dsl::researches
        .filter(research_dsl::pod_id.eq(pod.id))
        .get_results::<Research>(&*db);

    if pod_result.is_ok() {
        let researches = pod_result.unwrap();
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
            list_entry.current_level = Some(research.level);
        }
    }

    ok().message("Research data.").data(json!(&research_list))
}

#[post("/pod/add_research", data = "<queue_entry>", format = "application/json")]
pub fn add_research_to_queue(queue_entry: Result<JSON<QueueAddResearchSerializer>, SerdeError>,
                             current_user: User,
                             db: DB)
                             -> APIResponse {

    match queue_entry {
        // Return specific error if invalid JSON has been sent.
        Err(error) => return bad_request().message(format!("{}", error).as_str()),
        Ok(entry) => {
            // Check if the given research name maps to a research type.
            let result = ResearchTypes::from_string(&entry.research_name);
            match result {
                // Early return if we don't know this research name
                Err(_) => {
                    return bad_request().message(format!("No such research type `{}`",
                                                         entry.research_name)
                                                         .as_str());
                }
                Ok(research_type) => {
                    let dependency_strings = get_research_dependency_strings(&research_type);
                    let mut research_level: i32;
                    let research_list = get_research_list();
                    // Get pod and queue from db
                    let pod = pods_dsl::pods
                        .filter(pods_dsl::user_id.eq(current_user.id))
                        .first::<Pod>(&*db)
                        .unwrap();

                    let research = research_dsl::researches
                        .filter(research_dsl::pod_id.eq(pod.id))
                        .filter(research_dsl::name.eq(research_type.to_string()))
                        .get_result::<Research>(&*db);
                    match research {
                        // Research exists, we don't need to check for dependencies
                        // We just increase the level by 1
                        Ok(research) => {
                            research_level = research.level + 1;
                        }
                        // The research is not yet here. We need to check for dependencies.
                        // We just increase the level by 1
                        Err(_) => {
                            let dependencies = research_dsl::researches
                                .filter(research_dsl::name.eq_any(dependency_strings))
                                .get_results::<Research>(&*db);

                            let fulfilled = dependencies_fulfilled(&research_type,
                                                                   dependencies,
                                                                   &research_list);
                            if !fulfilled {
                                return bad_request().message("Dependencies not fulfilled.");
                            }
                            research_level = 1;
                        }
                    }

                    let queue = queues_dsl::queues
                        .filter(queues_dsl::pod_id.eq(pod.id))
                        .first::<Queue>(&*db)
                        .unwrap();

                    // Check if there already are existing queue entries for this research.
                    // In case there are, we increase the level by the amount of existing entries.
                    let existing_entries: i64 = queue_entries_dsl::queue_entries
                        .count()
                        .filter(queue_entries_dsl::queue_id.eq(queue.id))
                        .filter(queue_entries_dsl::research_name.eq(research_type.to_string()))
                        .get_result(&*db)
                        .unwrap_or(0);

                    let pod_resources = resources_dsl::resources
                        .filter(resources_dsl::pod_id.eq(pod.id))
                        .get_results(&*db)
                        .expect("Failed to get user resources.");

                    research_level += existing_entries as i32;

                    let all_levels = &research_list
                                          .get(&research_type)
                                          .as_ref()
                                          .expect("No research in yml for this type.")
                                          .level;

                    if !(all_levels.len() <= research_level as usize) {
                        return bad_request().message("Already at max level.");
                    }
                    let costs = &all_levels[research_level as usize].resources;
                    if costs.is_some() && !Resource::check_resources(costs, pod_resources, &db) {
                        return bad_request().message("Insufficient resources.");
                    }

                    // Create a new queue entry with the given research type.
                    let new_entry_model = NewQueueEntry {
                        queue_id: queue.id.clone(),
                        module_name: None,
                        research_name: Some(entry.research_name.clone()),
                        level: research_level,
                    };

                    let new_queue_entry = diesel::insert(&new_entry_model)
                        .into(queue_entries::table)
                        .get_result::<QueueEntry>(&*db)
                        .expect("Failed to update user.");

                    created()
                        .message("Queue entry added.")
                        .data(json!(&new_queue_entry))
                }
            }
        }
    }
}
