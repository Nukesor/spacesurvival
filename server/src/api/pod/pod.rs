
use diesel;
use diesel::prelude::*;
use rocket_contrib::{JSON, SerdeError};

use helpers::db::DB;
use validation::pod::PodSettingsSerializer;
use validation::queue::{QueueAddResearchSerializer, QueueAddModuleSerializer};

use schema::queue_entries;

use schema::pods::dsl as pods_dsl;
use schema::queues::dsl as queues_dsl;
use schema::resources::dsl as resource_dsl;
use schema::researches::dsl as research_dsl;
use schema::queue_entries::dsl::*;

use models::pod::{Pod, ChangedPod};
use models::user::User;
use models::queue::{QueueEntry, Queue, NewQueueEntry};
use models::research::Research;
use models::resource::Resource;

use data::helper::{get_research_dependency_strings, dependencies_fulfilled};
use data::types::{ResearchTypes, ModuleTypes};
use data::researches::get_research_list;

use responses::{APIResponse, bad_request, created, ok};


#[post("/settings", data = "<pod_settings>", format = "application/json")]
pub fn settings(pod_settings: Result<JSON<PodSettingsSerializer>, SerdeError>,
                current_user: User,
                db: DB)
                -> APIResponse {

    match pod_settings {
        // Return specific error if invalid JSON has been sent.
        Err(error) => return bad_request().message(format!("{}", error).as_str()),
        Ok(settings) => {
            // Get current pod
            let current_pod = pods_dsl::pods
                .filter(pods_dsl::user_id.eq(current_user.id))
                .first::<Pod>(&*db)
                .unwrap();

            // Create changed pod model and push it to the DB
            let changed_pod = ChangedPod { name: settings.name.clone() };
            let pod = diesel::update(pods_dsl::pods.filter(pods_dsl::id.eq(current_pod.id)))
                .set(&changed_pod)
                .get_result::<Pod>(&*db)
                .expect("Failed to update pod.");

            ok().message("Pod data changed.").data(json!(&pod))
        }
    }
}


#[post("/queue/add_research", data = "<queue_entry>", format = "application/json")]
pub fn add_research_to_queue(queue_entry: Result<JSON<QueueAddResearchSerializer>, SerdeError>,
                             current_user: User,
                             db: DB)
                             -> APIResponse {

    match queue_entry {
        // Return specific error if invalid JSON has been sent.
        Err(error) => return bad_request().message(format!("{}", error).as_str()),
        Ok(entry) => {
            // Check if the given research name maps to a research type.
            let result = ResearchTypes::from_str(entry.research_name.as_str());
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
                    let existing_entries: i64 = queue_entries
                        .count()
                        .filter(queue_id.eq(queue.id))
                        .filter(research_name.eq(research_type.to_string()))
                        .get_result(&*db)
                        .unwrap_or(0);

                    let pod_resources = resource_dsl::resources
                        .filter(resource_dsl::pod_id.eq(pod.id))
                        .get_results(&*db)
                        .expect("Failed to get user resources.");

                    research_level += existing_entries as i32;

                    let all_levels = &research_list.get(&research_type)
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

#[post("/queue/add_module", data = "<queue_entry>", format = "application/json")]
pub fn add_module_to_queue(queue_entry: Result<JSON<QueueAddModuleSerializer>, SerdeError>,
                           current_user: User,
                           db: DB)
                           -> APIResponse {

    match queue_entry {

        // Return specific error if invalid JSON has been sent.
        Err(error) => return bad_request().message(format!("{}", error).as_str()),
        Ok(entry) => {
            // Get pod and queue from db
            let pod = pods_dsl::pods
                .filter(pods_dsl::user_id.eq(current_user.id))
                .first::<Pod>(&*db)
                .unwrap();

            let queue = queues_dsl::queues
                .filter(queues_dsl::pod_id.eq(pod.id))
                .first::<Queue>(&*db)
                .unwrap();

            // Check if the given module name maps to a module type.
            if ModuleTypes::from_str(entry.module_name.as_str()).is_err() {
                return bad_request().message(format!("No such module type `{}`",
                                                     entry.module_name)
                                                     .as_str());
            }

            // Create a new queue entry with the given module type.
            let new_entry_model = NewQueueEntry {
                queue_id: queue.id.clone(),
                module_name: Some(entry.module_name.clone()),
                research_name: None,
                level: entry.level.clone(),
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
