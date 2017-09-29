use diesel;
use diesel::prelude::*;
use rocket_contrib::{Json, SerdeError};
use uuid::Uuid;

use data::types::*;
use data::modules::get_module_list;
use data::helper::{get_module_dependency_strings, dependencies_fulfilled};
use helpers::db::DB;
use helpers::request::validate_json;
use responses::{APIResponse, accepted, bad_request, created, ok, internal_server_error};
use validation::queue::NewModuleSerializer;

use models::user::User;
use models::module::{NewModule, Module};
use models::research::Research;
use models::resource::Resource;
use models::queue::{QueueEntry, NewQueueEntry};

use schema::modules;
use schema::modules::dsl as module_dsl;
use schema::researches::dsl as research_dsl;
use schema::queue_entries::dsl as queue_entry_dsl;


/// Add a new module to pod
///
/// This endpoint:
/// - Checks if dependencies for the module are fulfilled
/// - Checks if there are enough resources
/// - Removes resources from db
/// - Creates a new queue entry for this research
#[post("/pod/new", data = "<request_data>", format = "application/json")]
pub fn add_module(
    request_data: Result<Json<NewModuleSerializer>, SerdeError>,
    current_user: User,
    db: DB,
) -> Result<APIResponse, APIResponse> {
    // Unwrap or return specific error if invalid Json has been sent.
    let module_data = validate_json(request_data)?;

    // Check if the given module name maps to a module type.
    let result = ModuleTypes::from_string(&module_data.module_type);
    let module_type = result.or(Err(bad_request().error(
        "module",
        format!("No such module type `{}`", module_data.module_type).as_str(),
    )))?;
    let dependency_strings = get_module_dependency_strings(&module_type);

    let (pod, queue) = current_user.get_pod_and_queue(&db);

    let existing_module: i64;
    // Check if there already exists a module for this position
    // We distinguish between stationary and normal modules.

    // Stationary modules have unique names and only exists once
    // per pod.
    if module_data.stationary {
        existing_module = module_dsl::modules
            .count()
            .filter(module_dsl::pod_id.eq(pod.id))
            .filter(module_dsl::name.eq(&module_data.module_type))
            .get_result(&*db)
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
            .get_result(&*db)
            .unwrap_or(0);
    };

    // Early return if there already is a module.
    if existing_module != 0 {
        return Err(bad_request().error(
            "module",
            "There already is a module at this position.",
        ));
    }

    // Get the researches the module is depending on
    let dependencies = research_dsl::researches
        .filter(research_dsl::name.eq_any(dependency_strings))
        .get_results::<Research>(&*db);

    // Check if the dependencies are fulfilled.
    // If they're not fulfilled, return a bad request.
    let module_list = get_module_list();
    let fulfilled = dependencies_fulfilled(&module_type, dependencies, &module_list);
    if !fulfilled {
        return Err(bad_request().error("module", "Dependencies not fulfilled."));
    }

    // Get cost for level 1
    let level = &module_list
        .get(&module_type)
        .as_ref()
        .ok_or(internal_server_error())?
        .levels
        [0];

    // Check if we have enough resources and subtract them.
    if level.resources.is_some() && !pod.has_enough_resources(&level.resources, &db) {
        return Err(bad_request().error("module", "Insufficient resources."));
    }

    // Create the new module
    let new_module = NewModule {
        name: module_data.module_type.clone(),
        stationary: module_data.stationary,
        level: 0,
        x_pos: module_data.position_x,
        y_pos: module_data.position_y,

        pod_id: Some(pod.id),
        base_id: None,
    };
    let module = diesel::insert(&new_module)
        .into(modules::table)
        .get_result::<Module>(&*db)
        .or(Err(internal_server_error()))?;

    // Create a new queue entry with the given module type.
    let new_queue_entry = NewQueueEntry {
        queue_id: queue.id.clone(),
        research_id: None,
        research_name: None,
        module_name: Some(module.name),
        module_id: Some(module.id),
        level: 1,
        duration: level.time,
    };
    queue.add_entry(new_queue_entry, &db);

    Ok(created().message("Queue entry added."))
}


/// Remove module from pod
#[delete("/pod/<module_uuid>")]
pub fn remove_module(
    module_uuid: String,
    current_user: User,
    db: DB,
) -> Result<APIResponse, APIResponse> {
    // Parse and check if we got a valid id
    let module_id = Uuid::parse_str(&module_uuid.as_str()).or(Err(
        bad_request().error(
            "module",
            "Got an invalid uuid",
        ),
    ))?;

    // Get the module and get it from the pod to ensure this is a request from
    // the owner of the module
    let pod = current_user.get_pod(&db);
    let module = pod.get_module(module_id, &db).or(Err(bad_request().error(
        "module",
        "No module with this id.",
    )))?;

    // Remove queue_entry from database
    diesel::delete(module_dsl::modules.filter(module_dsl::id.eq(module.id)))
        .execute(&*db)
        .or(Err(internal_server_error()))?;

    Ok(accepted().message("Module removed."))
}

/// upgrade module from pod
#[post("/pod/upgrade/<module_uuid>")]
pub fn upgrade_module(
    module_uuid: String,
    current_user: User,
    db: DB,
) -> Result<APIResponse, APIResponse> {
    // Parse and check if we got a valid id
    let module_id = Uuid::parse_str(&module_uuid.as_str()).or(Err(
        bad_request().error(
            "module",
            "Got an invalid uuid",
        ),
    ))?;
    let (pod, queue) = current_user.get_pod_and_queue(&db);

    // Get the module and get it from the pod to ensure this is a request from
    // the owner of the module
    let module = pod.get_module(module_id, &db).or(Err(bad_request().error(
        "module",
        "No module with this id.",
    )))?;
    let level = module.level + 1;

    // Get all needed info for resource manipulation
    let module_list = get_module_list();
    // Add resources from module to pod resources
    let module_type = ModuleTypes::from_string(&module.name).or(Err(
        internal_server_error(),
    ))?;
    let all_levels = &module_list.get(&module_type).unwrap().levels;

    // Check if there is a next level.
    if level > all_levels.len() as i32 {
        return Err(bad_request().error("module", "Already at max level"));
    }

    let level_index: usize = (level - 1) as usize;
    let costs = &all_levels[level_index].resources;

    if costs.is_some() && !pod.has_enough_resources(costs, &db) {
        return Err(bad_request().error("module", "Insufficient resources."));
    }

    // Create a new queue entry with the given research type.
    let new_queue_entry = NewQueueEntry {
        queue_id: queue.id.clone(),
        research_id: None,
        research_name: None,
        module_name: Some(module.name),
        module_id: Some(queue.id.clone()),
        level: level,
        duration: all_levels[level_index].time,
    };

    queue.add_entry(new_queue_entry, &db);

    Ok(created().message("Queue entry added."))
}


/// Remove module upgrade from pod queue
#[delete("/pod/upgrade/<module_uuid>")]
pub fn stop_module_upgrade(
    module_uuid: String,
    current_user: User,
    db: DB,
) -> Result<APIResponse, APIResponse> {
    // Parse and check if we got a valid id
    let module_id = Uuid::parse_str(&module_uuid.as_str()).or(Err(
        bad_request().error(
            "module",
            "Got an invalid uuid",
        ),
    ))?;
    let (pod, queue) = current_user.get_pod_and_queue(&db);

    // Get the queue entry
    let queue_entry = queue_entry_dsl::queue_entries
        .filter(queue_entry_dsl::module_id.eq(module_id))
        .filter(queue_entry_dsl::queue_id.eq(queue.id))
        .order(queue_entry_dsl::level.desc())
        .first::<QueueEntry>(&*db)
        .or(Err(bad_request().error(
            "module",
            "No queue entry with this id.",
        )))?;

    // Get the module and get it from the pod to ensure this is a request from
    // the owner of the module
    let module = pod.get_module(module_id, &db).or(
        Err(internal_server_error()),
    )?;

    // Get all needed info for resource manipulation
    let module_list = get_module_list();
    let module_type = ModuleTypes::from_string(&module.name).or(Err(
        internal_server_error(),
    ))?;
    let all_levels = &module_list.get(&module_type).unwrap().levels;
    let costs_result = &all_levels[module.level as usize].resources;

    // Refund resources
    let pod_resources = pod.get_resources(&db);
    if let Some(ref costs) = *costs_result {
        Resource::change_resources(costs, pod_resources, false, &db);
    }

    queue.remove_entry(queue_entry.id, &db);

    // Remove module if it's just a upgrade dummy.
    if module.level == 0 {
        diesel::delete(module_dsl::modules.filter(module_dsl::id.eq(module.id)))
            .execute(&*db)
            .or(Err(internal_server_error()))?;
    }

    Ok(accepted().message("Module upgrade stopped."))
}
