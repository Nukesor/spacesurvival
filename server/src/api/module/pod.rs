use diesel;
use diesel::prelude::*;
use rocket_contrib::{JSON, SerdeError};

use uuid::Uuid;
use chrono::{UTC, Duration};

use data::types::*;
use data::modules::get_module_list;
use data::helper::{get_module_dependency_strings, dependencies_fulfilled};

use helpers::db::DB;
use responses::{APIResponse, bad_request, created, ok};

use models::user::User;
use models::module::{NewModule,Module};
use models::research::Research;
use models::resource::Resource;

use schema::modules;
use schema::queue_entries;

use schema::modules::dsl as module_dsl;
use schema::researches::dsl as research_dsl;
use schema::queue_entries::dsl as queue_entries_dsl;

use models::queue::{QueueEntry, NewQueueEntry};
use validation::queue::NewModuleSerializer;


/// The user needs to be logged in to access this route!
///
/// This route returns the list of all modules and their current levels for the pod of the current user.
#[get("/pod")]
pub fn get_modules(current_user: User, db: DB) -> APIResponse {

    let pod = current_user.get_pod(&db);
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
/// - Creates a new queue entry for this research
#[post("/pod/new", data = "<request_data>", format = "application/json")]
pub fn add_module(request_data: Result<JSON<NewModuleSerializer>, SerdeError>,
                          current_user: User,
                          db: DB)
                          -> APIResponse {

    // Unwrap or return specific error if invalid JSON has been sent.
    if let Err(error) = request_data {
        return bad_request().message(format!("{}", error).as_str());
    };
    let module_data = request_data.unwrap();

    // Check if the given module name maps to a module type.
    let result = ModuleTypes::from_string(&module_data.module_name);
    if result.is_err() {
        return bad_request().message(format!("No such module type `{}`",
                                             module_data.module_name)
                                             .as_str());
    }
    let module_type = result.unwrap();
    let dependency_strings = get_module_dependency_strings(&module_type);

    let (pod, queue) = current_user.get_pod_and_queue(&db);

    let existing_module;
    // Check if there already exists a module for this position
    // We distinguish between stationary and normal modules.

    // Stationary modules have unique names and only exists once
    // per pod.
    if module_data.stationary {
        existing_module = module_dsl::modules
            .count()
            .filter(module_dsl::pod_id.eq(pod.id))
            .filter(module_dsl::name.eq(&module_data.module_name))
            .execute(&*db)
            .unwrap_or(0);
    }
    // Normal modules can exist multiple times. Thereby we just 
    // have to check if the requested position is free.
    else {
        existing_module = module_dsl::modules
            .count()
            .filter(module_dsl::pod_id.eq(pod.id))
            .filter(module_dsl::x_pos.eq(module_data.position_x))
            .filter(module_dsl::y_pos.eq(module_data.position_y))
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
    let module_list = get_module_list();
    let fulfilled = dependencies_fulfilled(&module_type,
                                           dependencies,
                                           &module_list);
    if !fulfilled {
        return bad_request().message("Dependencies not fulfilled.");
    }

    // Get cost for level 1
    let level = &module_list.get(&module_type)
          .as_ref()
          .expect("No module in yml for this type.")
          .levels[0];

    // Check if we have enough resources and subtract them.
    let pod_resources = pod.get_resources(&db);
    if level.resources.is_some() && !Resource::check_resources(&level.resources, pod_resources, &db) {
        return bad_request().message("Insufficient resources.");
    }

    // Create the new module
    let new_module = NewModule {
        name: module_data.module_name.clone(),
        stationary: module_data.stationary,
        x_pos: module_data.position_x,
        y_pos: module_data.position_y,

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
        research_id: None,
        research_name: None,
        module_name: Some(module.name),
        module_id: Some(module.id),
        level: 1,
        finishes_at: (UTC::now() + Duration::seconds(level.time)).naive_utc(),
    };
    let new_queue_entry = diesel::insert(&new_entry_model)
        .into(queue_entries::table)
        .get_result::<QueueEntry>(&*db)
        .expect("Failed to create queue entry.");

    created()
        .message("Queue entry added.")
        .data(json!(&new_queue_entry))
}


/// Remove module from pod
#[delete("/pod/<module_uuid>")]
pub fn remove_module(module_uuid: &str, current_user: User, db: DB) -> APIResponse {
    // Parse and check if we got a valid id
    let result = Uuid::parse_str(module_uuid);
    if result.is_err() {
        return bad_request().message("Got an invalid uuid");
    }
    let module_id = result.unwrap();

    let pod = current_user.get_pod(&db);

    // Get the module
    let module_result = module_dsl::modules
        .filter(module_dsl::id.eq(module_id))
        .filter(module_dsl::pod_id.eq(pod.id))
        .first::<Module>(&*db);
    if module_result.is_err() {
        return bad_request().message("No module with this id.");
    }
    let module = module_result.unwrap();

    // Remove queue_entry from database
    diesel::delete(module_dsl::modules
                       .filter(module_dsl::id.eq(module.id)))
            .execute(&*db)
            .expect("Failed to remove module.");

    ok().message("Module removed.")
}

/// upgrade module from pod
#[post("/pod/upgrade/<module_uuid>")]
pub fn upgrade_module(module_uuid: &str, current_user: User, db: DB) -> APIResponse {
    // Parse and check if we got a valid id
    let result = Uuid::parse_str(module_uuid);
    if result.is_err() {
        return bad_request().message("Got an invalid uuid");
    }
    let module_id = result.unwrap();
    let pod = current_user.get_pod(&db);

    // Get the module
    let module_result = module_dsl::modules
        .filter(module_dsl::id.eq(module_id))
        .filter(module_dsl::pod_id.eq(pod.id))
        .first::<Module>(&*db);
    if module_result.is_err() {
        return bad_request().message("No module with this id.");
    }
    let module = module_result.unwrap();
    let level = module.level + 1;

    // Get all needed info for resource manipulation
    let module_list = get_module_list();

    let (pod, queue) = current_user.get_pod_and_queue(&db);
    let pod_resources = pod.get_resources(&db);

    // Add resources from module to pod resources
    let all_levels = &module_list
                          .get(&ModuleTypes::from_string(&module.name).unwrap())
                          .unwrap()
                          .levels;

    // Check if there is a next level.
    if level > all_levels.len() as i32 {
        return bad_request().message("Already at max level");
    }


    let level_index: usize = (level-1) as usize;
    let costs = &all_levels[level_index].resources;

    if costs.is_some() && !Resource::check_resources(costs, pod_resources, &db) {
        return bad_request().message("Insufficient resources.");
    }

    // Create a new queue entry with the given research type.
    let new_entry_model = NewQueueEntry {
        queue_id: queue.id.clone(),
        research_id: None,
        research_name: None,
        module_name: Some(module.name),
        module_id: Some(queue.id.clone()),
        level: level,
        finishes_at: (UTC::now() + Duration::seconds(all_levels[level_index].time)).naive_utc(),
    };

    let new_queue_entry = diesel::insert(&new_entry_model)
        .into(queue_entries::table)
        .get_result::<QueueEntry>(&*db)
        .expect("Failed to insert new queue entry.");

    created()
        .message("Queue entry added.")
        .data(json!(&new_queue_entry))
}


/// Remove module upgrade from pod queue
#[delete("/pod/upgrade/<module_uuid>")]
pub fn stop_module_upgrade(module_uuid: &str, current_user: User, db: DB) -> APIResponse {
    // Parse and check if we got a valid id
    let result = Uuid::parse_str(module_uuid);
    if result.is_err() {
        return bad_request().message("Got an invalid uuid");
    }
    let module_id = result.unwrap();
    let (pod, queue) = current_user.get_pod_and_queue(&db);

    // Get the queue entry
    let queue_entry_result = queue_entries_dsl::queue_entries
        .filter(queue_entries_dsl::module_id.eq(module_id))
        .filter(queue_entries_dsl::queue_id.eq(queue.id))
        .order(queue_entries_dsl::level.desc())
        .first::<QueueEntry>(&*db);
    if queue_entry_result.is_err() {
        return bad_request().message("No queue entry with this id.");
    }
    let queue_entry = queue_entry_result.unwrap();

    // Get the module from the queue entry
    let module = module_dsl::modules
        .filter(module_dsl::id.eq(module_id))
        .get_result::<Module>(&*db)
        .expect("Failed to get a module");

    // Get all needed info for resource manipulation
    let module_list = get_module_list();
    let all_levels = &module_list
                          .get(&ModuleTypes::from_string(&module.name).unwrap())
                          .unwrap()
                          .levels;
    let costs_result = &all_levels[module.level as usize].resources;

    // Refund resources
    let pod_resources = pod.get_resources(&db);
    if let Some(ref costs) = *costs_result {
        Resource::update_resources(costs, pod_resources, false, &db);
    }

    // Remove queue_entry from database
    diesel::delete(queue_entries_dsl::queue_entries
                       .filter(queue_entries_dsl::id.eq(queue_entry.id)))
            .execute(&*db)
            .expect("Failed to remove queue_entry.");

    // Remove module if it's just a upgrade dummy.
    if module.level == 0 {
        diesel::delete(module_dsl::modules
                       .filter(module_dsl::id.eq(module.id)))
            .execute(&*db)
            .expect("Failed to remove module.");
    }

    ok().message("Module upgrade stopped.")
}
