use diesel;
use diesel::prelude::*;
use rocket_contrib::{JSON, SerdeError};
use uuid::Uuid;

use data::types::*;
use data::modules::get_module_list;
use data::helper::{get_module_dependency_strings, dependencies_fulfilled};

use helpers::db::DB;
use responses::{APIResponse, bad_request, created, ok};

use models::pod::Pod;
use models::user::User;
use models::module::Module;
use models::resource::Resource;

use schema::queue_entries;
use schema::pods::dsl as pods_dsl;
use schema::queues::dsl as queues_dsl;
use schema::resources::dsl as resources_dsl;
use schema::modules::dsl as module_dsl;
use schema::queue_entries::dsl as queue_entries_dsl;

use models::queue::{QueueEntry, Queue, NewQueueEntry};
use validation::queue::{NewModuleSerializer, UpgradeModuleSerializer};


/// The user needs to be logged in to access this route!
///
/// This route returns the list of all modules and their current levels for the pod of the current user.
//#[get("/pod")]
//pub fn get_modules(current_user: User, db: DB) -> APIResponse {
//
//    let mut module_list = get_module_list();
//    // Create changed pod model and push it to the DB
//
//    let pod = pods_dsl::pods
//        .filter(pods_dsl::user_id.eq(current_user.id))
//        .get_result::<Pod>(&*db)
//        .expect("Failed to get user pod.");
//
//    let pod_result = module_dsl::modules
//        .filter(module_dsl::pod_id.eq(pod.id))
//        .get_results::<Module>(&*db);
//
//    if pod_result.is_ok() {
//        let modules = pod_result.unwrap();
//        for module in modules {
//            let type_result = ModuleTypes::from_string(&module.name);
//            if type_result.is_err() {
//                return bad_request()
//                           .message(format!("Found module {}, but no matching ModuleType!",
//                                            module.name)
//                                            .as_str());
//            }
//            let module_type = type_result.unwrap();
//            let list_result = module_list.get_mut(&module_type);
//            if list_result.is_none() {
//                return bad_request().message(format!("Found type {}, but no matching entry in our module list!", module_type).as_str());
//            }
//
//            let mut list_entry = list_result.unwrap();
//            list_entry.current_level = Some(module.level);
//        }
//    }
//
//    ok().message("Module data.").data(json!(&module_list))
//}

/// Add a new module to pod
///
/// This endpoint:
/// - Checks if dependencies for module are fulfilled
/// - Checks if there are enough resources
/// - Removes resources from db
#[post("/pod/new", data = "<new_module_dat>", format = "application/json")]
pub fn add_module(new_module_data: Result<JSON<NewModuleSerializer>, SerdeError>,
                          current_user: User,
                          db: DB)
                          -> APIResponse {

    match new_module_data {
        // Return specific error if invalid JSON has been sent.
        Err(error) => return bad_request().message(format!("{}", error).as_str()),
        Ok(entry) => {
            // Check if the given module name maps to a module type.
            let result = ModuleTypes::from_string(&entry.module_name);
            match result {
                // Early return if we don't know this module name
                Err(_) => {
                    return bad_request().message(format!("No such module type `{}`",
                                                         entry.module_name)
                                                         .as_str());
                }
                Ok(module_type) => {
                    let dependency_strings = get_module_dependency_strings(&module_type);
                    let mut module_level: i32;
                    let module_list = get_module_list();
                    // Get pod and queue from db
                    let pod = pods_dsl::pods
                        .filter(pods_dsl::user_id.eq(current_user.id))
                        .first::<Pod>(&*db)
                        .unwrap();

                    let module = module_dsl::modules
                        .filter(module_dsl::pod_id.eq(pod.id))
                        .filter(module_dsl::name.eq(module_type.to_string()))
                        .get_result::<Module>(&*db);
                    match module {
                        // Module exists, we don't need to check for dependencies
                        // We just increase the level by 1
                        Ok(module) => {
                            module_level = module.level + 1;
                        }
                        // The module is not yet here. We need to check for dependencies.
                        // We just increase the level by 1
                        Err(_) => {
                            let dependencies = module_dsl::modules
                                .filter(module_dsl::name.eq_any(dependency_strings))
                                .get_results::<Module>(&*db);

                            let fulfilled = dependencies_fulfilled(&module_type,
                                                                   dependencies,
                                                                   &module_list);
                            if !fulfilled {
                                return bad_request().message("Dependencies not fulfilled.");
                            }
                            module_level = 1;
                        }
                    }

                    let queue = queues_dsl::queues
                        .filter(queues_dsl::pod_id.eq(pod.id))
                        .first::<Queue>(&*db)
                        .unwrap();

                    // Check if there already are existing queue entries for this module.
                    // In case there are, we increase the level by the amount of existing entries.
                    let existing_entries: i64 = queue_entries_dsl::queue_entries
                        .count()
                        .filter(queue_entries_dsl::queue_id.eq(queue.id))
                        .filter(queue_entries_dsl::module_name.eq(module_type.to_string()))
                        .get_result(&*db)
                        .unwrap_or(0);

                    let pod_resources = resources_dsl::resources
                        .filter(resources_dsl::pod_id.eq(pod.id))
                        .get_results(&*db)
                        .expect("Failed to get user resources.");

                    module_level += existing_entries as i32;

                    let all_levels = &module_list
                                          .get(&module_type)
                                          .as_ref()
                                          .expect("No module in yml for this type.")
                                          .levels;

                    if !(all_levels.len() <= module_level as usize) {
                        return bad_request().message("Already at max level.");
                    }
                    let costs = &all_levels[module_level as usize].resources;
                    if costs.is_some() && !Resource::check_resources(costs, pod_resources, &db) {
                        return bad_request().message("Insufficient resources.");
                    }

                    // Create a new queue entry with the given module type.
                    let new_entry_model = NewQueueEntry {
                        queue_id: queue.id.clone(),
                        module_name: None,
                        module_name: Some(entry.module_name.clone()),
                        level: module_level,
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


/// Remove module from pod
//#[delete("/pod/<entry_uuid>")]
//pub fn remove_module(entry_uuid: &str, current_user: User, db: DB) -> APIResponse {
//
//    // Parse and check if we got a valid id
//    let result = Uuid::parse_str(entry_uuid);
//    if result.is_err() {
//        return bad_request().message("Got an invalid uuid");
//    }
//    let queue_entry_id = result.unwrap();
//
//    // Get the queue entry
//    let queue_entry_result = queue_entries_dsl::queue_entries
//        .filter(queue_entries_dsl::id.eq(queue_entry_id))
//        .first::<QueueEntry>(&*db);
//    if queue_entry_result.is_err() {
//        return bad_request().message("No queue entry with this id.");
//    }
//    let queue_entry = queue_entry_result.unwrap();
//
//    // Check if we got an module queue entry
//    if queue_entry.module_name.is_none() {
//        return bad_request().message("Queue entry is a model queue entry.");
//    }
//
//    // Check if there already are existing queue entries for this module.
//    // If there are entries with a higher level, we return a bad request.
//    let level = queue_entry.level;
//    let name = queue_entry.module_name.unwrap();
//    let higher_entry = queue_entries_dsl::queue_entries
//        .filter(queue_entries_dsl::queue_id.eq(queue_entry.queue_id))
//        .filter(queue_entries_dsl::module_name.eq(&name))
//        .filter(queue_entries_dsl::level.gt(queue_entry.level))
//        .get_result::<QueueEntry>(&*db);
//    if higher_entry.is_ok() {
//        return bad_request().message("Can't delete. There is an queue entry with a higher level for this module.");
//    }
//
//    // Get all needed info for resource manipulation
//    let module_list = get_module_list();
//
//    let pod = pods_dsl::pods
//        .filter(pods_dsl::user_id.eq(current_user.id))
//        .first::<Pod>(&*db)
//        .unwrap();
//
//    let pod_resources = resources_dsl::resources
//        .filter(resources_dsl::pod_id.eq(pod.id))
//        .get_results::<Resource>(&*db)
//        .expect("Failed to get user resources.");
//
//    // Add resources from module to pod resources
//    let all_levels = &module_list
//                          .get(&ModuleTypes::from_string(&name).unwrap())
//                          .unwrap()
//                          .levels;
//    let costs_result = &all_levels[level as usize].resources;
//
//    if let Some(ref costs) = *costs_result {
//        Resource::update_resources(costs, pod_resources, false, &db);
//    }
//
//    // Remove queue_entry from database
//    diesel::delete(queue_entries_dsl::queue_entries
//                       .filter(queue_entries_dsl::id.eq(queue_entry_id)))
//            .execute(&*db)
//            .expect("Failed to remove queue_entry.");
//
//    ok().message("Resource removed.")
//}
//
///// upgrade module from pod
//#[post("/pod/upgrade/<entry_uuid>")]
//pub fn upgrade_module(entry_uuid: &str, current_user: User, db: DB) -> APIResponse {
//
//    // Parse and check if we got a valid id
//    let result = Uuid::parse_str(entry_uuid);
//    if result.is_err() {
//        return bad_request().message("Got an invalid uuid");
//    }
//    let queue_entry_id = result.unwrap();
//
//    // Get the queue entry
//    let queue_entry_result = queue_entries_dsl::queue_entries
//        .filter(queue_entries_dsl::id.eq(queue_entry_id))
//        .first::<QueueEntry>(&*db);
//    if queue_entry_result.is_err() {
//        return bad_request().message("No queue entry with this id.");
//    }
//    let queue_entry = queue_entry_result.unwrap();
//
//    // Check if we got an module queue entry
//    if queue_entry.module_name.is_none() {
//        return bad_request().message("Queue entry is a model queue entry.");
//    }
//
//    // Check if there already are existing queue entries for this module.
//    // If there are entries with a higher level, we return a bad request.
//    let level = queue_entry.level;
//    let name = queue_entry.module_name.unwrap();
//    let higher_entry = queue_entries_dsl::queue_entries
//        .filter(queue_entries_dsl::queue_id.eq(queue_entry.queue_id))
//        .filter(queue_entries_dsl::module_name.eq(&name))
//        .filter(queue_entries_dsl::level.gt(queue_entry.level))
//        .get_result::<QueueEntry>(&*db);
//    if higher_entry.is_ok() {
//        return bad_request().message("Can't delete. There is an queue entry with a higher level for this module.");
//    }
//
//    // Get all needed info for resource manipulation
//    let module_list = get_module_list();
//
//    let pod = pods_dsl::pods
//        .filter(pods_dsl::user_id.eq(current_user.id))
//        .first::<Pod>(&*db)
//        .unwrap();
//
//    let pod_resources = resources_dsl::resources
//        .filter(resources_dsl::pod_id.eq(pod.id))
//        .get_results::<Resource>(&*db)
//        .expect("Failed to get user resources.");
//
//    // Add resources from module to pod resources
//    let all_levels = &module_list
//                          .get(&ModuleTypes::from_string(&name).unwrap())
//                          .unwrap()
//                          .levels;
//    let costs_result = &all_levels[level as usize].resources;
//
//    if let Some(ref costs) = *costs_result {
//        Resource::update_resources(costs, pod_resources, false, &db);
//    }
//
//    // Remove queue_entry from database
//    diesel::delete(queue_entries_dsl::queue_entries
//                       .filter(queue_entries_dsl::id.eq(queue_entry_id)))
//            .execute(&*db)
//            .expect("Failed to remove queue_entry.");
//
//    ok().message("Resource removed.")
//}
//
//
///// Remove module upgrade from pod queue
//#[delete("/pod/upgrade/<entry_uuid>")]
//pub fn stop_module_upgrade(entry_uuid: &str, current_user: User, db: DB) -> APIResponse {
//
//    // Parse and check if we got a valid id
//    let result = Uuid::parse_str(entry_uuid);
//    if result.is_err() {
//        return bad_request().message("Got an invalid uuid");
//    }
//    let queue_entry_id = result.unwrap();
//
//    // Get the queue entry
//    let queue_entry_result = queue_entries_dsl::queue_entries
//        .filter(queue_entries_dsl::id.eq(queue_entry_id))
//        .first::<QueueEntry>(&*db);
//    if queue_entry_result.is_err() {
//        return bad_request().message("No queue entry with this id.");
//    }
//    let queue_entry = queue_entry_result.unwrap();
//
//    // Check if we got an module queue entry
//    if queue_entry.module_id.is_none() {
//        return bad_request().message("Queue entry is a model queue entry.");
//    }
//
//    // Check if there already are existing queue entries for this module.
//    // If there are entries with a higher level, we return a bad request.
//    let level = queue_entry.level;
//    let name = queue_entry.module_name.unwrap();
//    let higher_entry = queue_entries_dsl::queue_entries
//        .filter(queue_entries_dsl::queue_id.eq(queue_entry.queue_id))
//        .filter(queue_entries_dsl::module_name.eq(&name))
//        .filter(queue_entries_dsl::level.gt(queue_entry.level))
//        .get_result::<QueueEntry>(&*db);
//    if higher_entry.is_ok() {
//        return bad_request().message("Can't delete. There is an queue entry with a higher level for this module.");
//    }
//
//    // Get all needed info for resource manipulation
//    let module_list = get_module_list();
//
//    let pod = pods_dsl::pods
//        .filter(pods_dsl::user_id.eq(current_user.id))
//        .first::<Pod>(&*db)
//        .unwrap();
//
//    let pod_resources = resources_dsl::resources
//        .filter(resources_dsl::pod_id.eq(pod.id))
//        .get_results::<Resource>(&*db)
//        .expect("Failed to get user resources.");
//
//    // Add resources from module to pod resources
//    let all_levels = &module_list
//                          .get(&ModuleTypes::from_string(&name).unwrap())
//                          .unwrap()
//                          .levels;
//    let costs_result = &all_levels[level as usize].resources;
//
//    if let Some(ref costs) = *costs_result {
//        Resource::update_resources(costs, pod_resources, false, &db);
//    }
//
//    // Remove queue_entry from database
//    diesel::delete(queue_entries_dsl::queue_entries
//                       .filter(queue_entries_dsl::id.eq(queue_entry_id)))
//            .execute(&*db)
//            .expect("Failed to remove queue_entry.");
//
//    ok().message("Resource removed.")
//}
