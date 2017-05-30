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
use models::module::{NewModule,Module};
use models::research::Research;
use models::resource::Resource;

use schema::modules;
use schema::queue_entries;

use schema::pods::dsl as pods_dsl;
use schema::queues::dsl as queues_dsl;
use schema::modules::dsl as module_dsl;
use schema::resources::dsl as resources_dsl;
use schema::researches::dsl as research_dsl;

use models::queue::{QueueEntry, Queue, NewQueueEntry};
use validation::queue::{NewModuleSerializer, UpgradeModuleSerializer};


/// The user needs to be logged in to access this route!
///
/// This route returns the list of all modules and their current levels for the pod of the current user.
#[get("/pod")]
pub fn get_modules(current_user: User, db: DB) -> APIResponse {

    let pod = pods_dsl::pods
        .filter(pods_dsl::user_id.eq(current_user.id))
        .get_result::<Pod>(&*db)
        .expect("Failed to get user pod.");

    let module_result = module_dsl::modules
        .filter(module_dsl::pod_id.eq(pod.id))
        .get_results::<Module>(&*db);

    if module_result.is_ok() {
        let modules = module_result.unwrap();
        return ok().message("Module data.").data(json!(&modules));
    }
    ok().message("No module installed.")
}

/// Add a new module to pod
///
/// This endpoint:
/// - Checks if dependencies for the module are fulfilled
/// - Checks if there are enough resources
/// - Removes resources from db
#[post("/pod/new", data = "<new_module_data>", format = "application/json")]
pub fn add_module(new_module_data: Result<JSON<NewModuleSerializer>, SerdeError>,
                          current_user: User,
                          db: DB)
                          -> APIResponse {
    match new_module_data {
        // Return specific error if invalid JSON has been sent.
        Err(error) => return bad_request().message(format!("{}", error).as_str()),
        Ok(data) => {
            // Check if the given module name maps to a module type.
            let result = ModuleTypes::from_string(&data.module_name);
            match result {
                // Early return if we don't know this module name
                Err(_) => {
                    return bad_request().message(format!("No such module type `{}`",
                                                         data.module_name)
                                                         .as_str());
                }
                Ok(module_type) => {
                    let dependency_strings = get_module_dependency_strings(&module_type);
                    let module_list = get_module_list();

                    // Get pod and queue from db
                    let pod = pods_dsl::pods
                        .filter(pods_dsl::user_id.eq(current_user.id))
                        .first::<Pod>(&*db)
                        .unwrap();

                    let existing_module;
                    // Check if there already exists a module for this position
                    // We distinguish between stationary and normal modules.

                    // Stationary modules have unique names and only exists once
                    // per pod.
                    if data.stationary {
                        existing_module = module_dsl::modules
                            .count()
                            .filter(module_dsl::pod_id.eq(pod.id))
                            .filter(module_dsl::name.eq(&data.module_name))
                            .execute(&*db)
                            .unwrap_or(0);
                    }
                    // Normal modules can exist multiple times. Thereby we just 
                    // have to check if the requested position is free.
                    else {
                        existing_module = module_dsl::modules
                            .count()
                            .filter(module_dsl::pod_id.eq(pod.id))
                            .filter(module_dsl::x_pos.eq(data.position_x))
                            .filter(module_dsl::y_pos.eq(data.position_y))
                            .execute(&*db)
                            .unwrap_or(0);
                    };

                    // Early return if there already is a module.
                    if existing_module != 0 {
                        return bad_request().message("There already is a module at this position.");
                    }

                    // Get the researches the module is depending on
                    let dependencies = research_dsl::researches
                        .filter(research_dsl::name.eq_any(dependency_strings))
                        .get_results::<Research>(&*db);

                    // Check if the dependencies are fulfilled.
                    // If they're not fulfilled, return a bad request.
                    let fulfilled = dependencies_fulfilled(&module_type,
                                                           dependencies,
                                                           &module_list);
                    if !fulfilled {
                        return bad_request().message("Dependencies not fulfilled.");
                    }

                    let queue = queues_dsl::queues
                        .filter(queues_dsl::pod_id.eq(pod.id))
                        .first::<Queue>(&*db)
                        .unwrap();

                    // Query all pod resources
                    let pod_resources = resources_dsl::resources
                        .filter(resources_dsl::pod_id.eq(pod.id))
                        .get_results(&*db)
                        .expect("Failed to get user resources.");

                    // Get cost for level 1
                    let costs= &module_list
                                          .get(&module_type)
                                          .as_ref()
                                          .expect("No module in yml for this type.")
                                          .levels[0].resources;

                    if costs.is_some() {
                        if !Resource::check_resources(&costs, pod_resources, &db) {
                            return bad_request().message("Insufficient resources.");
                        }
                    }

                    // Create a new module in the
                    let new_module = NewModule {
                        name: data.module_name.clone(),
                        stationary: data.stationary,
                        x_pos: data.position_x,
                        y_pos: data.position_y,

                        pod_id: Some(pod.id),
                        base_id: None,
                    };

                    let module = diesel::insert(&new_module)
                        .into(modules::table)
                        .get_result::<Module>(&*db)
                        .expect("Failed to create module.");

                    // Create a new queue entry with the given module type.
                    let new_entry_model = NewQueueEntry {
                        queue_id: queue.id.clone(),
                        research_name: None,
                        module_name: Some(module.name),
                        module_id: Some(module.id),
                        level: 1,
                    };

                    let new_queue_entry = diesel::insert(&new_entry_model)
                        .into(queue_entries::table)
                        .get_result::<QueueEntry>(&*db)
                        .expect("Failed to create queue entry.");

                    created()
                        .message("Queue entry added.")
                        .data(json!(&new_queue_entry))
                }
            }
        }
    }
}


///// Remove module from pod
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

/// upgrade module from pod
#[post("/pod/upgrade/<entry_uuid>")]
pub fn upgrade_module(entry_uuid: &str, current_user: User, db: DB) -> APIResponse {

    // Parse and check if we got a valid id
    let result = Uuid::parse_str(entry_uuid);
    if result.is_err() {
        return bad_request().message("Got an invalid uuid");
    }
    let module_id = result.unwrap();

    // Get the module
    let module_result = module_dsl::modules
        .filter(module_dsl::id.eq(module_id))
        .first::<Module>(&*db);
    if module_result.is_err() {
        return bad_request().message("No module with this id.");
    }
    let module = module_result.unwrap();
    let level = module.level + 1;

    // Get all needed info for resource manipulation
    let module_list = get_module_list();

    let pod = pods_dsl::pods
        .filter(pods_dsl::user_id.eq(current_user.id))
        .first::<Pod>(&*db)
        .unwrap();

    let queue = queues_dsl::queues
        .filter(queues_dsl::pod_id.eq(pod.id))
        .first::<Queue>(&*db)
        .unwrap();


    let pod_resources = resources_dsl::resources
        .filter(resources_dsl::pod_id.eq(pod.id))
        .get_results::<Resource>(&*db)
        .expect("Failed to get user resources.");

    // Add resources from module to pod resources
    let all_levels = &module_list
                          .get(&ModuleTypes::from_string(&module.name).unwrap())
                          .unwrap()
                          .levels;

    // Check if there is a next level.
    if level >= all_levels.len() as i32 {
        return bad_request().message("Already at max level");
    }

    let costs_result = &all_levels[level as usize].resources;

    if let Some(ref costs) = *costs_result {
        Resource::update_resources(costs, pod_resources, false, &db);
    }

    // Create a new queue entry with the given research type.
    let new_entry_model = NewQueueEntry {
        queue_id: queue.id.clone(),
        research_name: None,
        module_name: Some(module.name),
        module_id: Some(queue.id.clone()),
        level: level,
    };

    let new_queue_entry = diesel::insert(&new_entry_model)
        .into(queue_entries::table)
        .get_result::<QueueEntry>(&*db)
        .expect("Failed to insert new queue entry.");

    created()
        .message("Queue entry added.")
        .data(json!(&new_queue_entry))
}


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
