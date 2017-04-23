use std::ops::Deref;

use diesel;
use diesel::prelude::*;
use rocket_contrib::{JSON, SerdeError};

use helpers::db::DB;
use validation::pod::{PodSettingsSerializer};
use validation::queue::{QueueAddResearchSerializer, QueueAddModuleSerializer};

use schema::{pods, queue_entries};
use schema::pods::dsl::*;
use schema::queues::dsl::*;
use schema::queue_entries::dsl::*;
use schema::researches::dsl as research_dsl;

use models::pod::{PodModel, ChangedPod};
use models::user::UserModel;
use models::queue::{QueueEntryModel, QueueModel, NewQueueEntry};
use models::research::ResearchModel;

use data::helper::{
    get_research_dependency_strings,
    dependencies_fulfilled,
};
use data::types::{ResearchTypes, ModuleTypes};
use data::researches::RESEARCH_LIST;

use responses::{
    APIResponse,
    bad_request,
    created,
    ok, 
};


#[post("/settings", data = "<pod_settings>", format = "application/json")]
pub fn settings(pod_settings: Result<JSON<PodSettingsSerializer>, SerdeError>, current_user: UserModel, db: DB) -> APIResponse {

    match pod_settings {
        // Return specific error if invalid JSON has been sent.
        Err(error) => return bad_request().message(format!("{}", error).as_str()),
        Ok(settings) =>  {
            // Get current pod
            let current_pod = pods.filter(user_id.eq(current_user.id))
                .first::<PodModel>(&*db)
                .unwrap();

            // Create changed pod model and push it to the DB
            let changed_pod = ChangedPod {
                name : settings.name.clone(),
            };
            let pod = diesel::update(pods.filter(pods::id.eq(current_pod.id)))
                .set(&changed_pod)
                .get_result::<PodModel>(&*db)
                .expect("Failed to update pod.");

            ok().message("Pod data changed.").data(json!(&pod))
        }
    }
}


#[post("/queue/add_research", data = "<queue_entry>", format = "application/json")]
pub fn add_research_to_queue(queue_entry: Result<JSON<QueueAddResearchSerializer>, SerdeError>, current_user: UserModel, db: DB) -> APIResponse {

    match queue_entry {
        // Return specific error if invalid JSON has been sent.
        Err(error) => return bad_request().message(format!("{}", error).as_str()),
        Ok(entry) =>  {
            // Check if the given research name maps to a research type.
            let result = ResearchTypes::from_str(entry.research_name.as_str());
            match result {
                // Early return if we don't know this research name
                Err(_) => { 
                    return bad_request()
                        .message(format!("No such research type `{}`", entry.research_name).as_str());
                    },
                Ok(research_type) => {
                    let dependency_strings = get_research_dependency_strings(&research_type);
                    let mut research_level: i32;
                    // Get pod and queue from db
                    let pod = pods.filter(user_id.eq(current_user.id))
                        .first::<PodModel>(&*db)
                        .unwrap();

                    let research = research_dsl::researches
                        .filter(research_dsl::pod_id.eq(pod.id))
                        .filter(research_dsl::name.eq(research_type.to_string()))
                        .get_result::<ResearchModel>(&*db);
                    match research {
                        // Research exists, we don't need to check for dependencies
                        // We just increase the level by 1
                        Ok(research) => {
                            research_level = research.level + 1;
                        },
                        // The research is not yet here. We need to check for dependencies.
                        // We just increase the level by 1
                        Err(_) => {
                            research_level = 1;
                        }
                    }

                    let dependencies = research_dsl::researches
                        .filter(research_dsl::name.eq_any(dependency_strings))
                        .get_results::<ResearchModel>(&*db);

                    let fulfilled = dependencies_fulfilled(
                        &research_type,
                        dependencies,
                        RESEARCH_LIST.deref()
                    );
                    if !fulfilled {
                        return bad_request().message("Dependencies not fulfilled.")
                    }

                    let queue = queues.filter(pod_id.eq(pod.id))
                        .first::<QueueModel>(&*db)
                        .unwrap();

                    let existing_entries: i64 = queue_entries.count()
                        .filter(queue_id.eq(queue.id))
                        .filter(research_name.eq(research_type.to_string()))
                        .get_result(&*db)
                        .unwrap_or(0);

                    research_level += existing_entries as i32;

                    // Create a new queue entry with the given research type.
                    let new_entry_model = NewQueueEntry {
                        queue_id: queue.id.clone(),
                        module_name: None,
                        research_name: Some(entry.research_name.clone()),
                        level: research_level,
                    };

                    let new_queue_entry = diesel::insert(&new_entry_model)
                        .into(queue_entries::table)
                        .get_result::<QueueEntryModel>(&*db)
                        .expect("Failed to update user.");

                    created().message("Queue entry added.").data(json!(&new_queue_entry))
                }
            }
        }
    }
}

#[post("/queue/add_module", data = "<queue_entry>", format = "application/json")]
pub fn add_module_to_queue(queue_entry: Result<JSON<QueueAddModuleSerializer>, SerdeError>, current_user: UserModel, db: DB) -> APIResponse {

    match queue_entry {

        // Return specific error if invalid JSON has been sent.
        Err(error) => return bad_request().message(format!("{}", error).as_str()),
        Ok(entry) =>  {
            // Get pod and queue from db
            let pod = pods.filter(user_id.eq(current_user.id))
                .first::<PodModel>(&*db)
                .unwrap();

            let queue = queues.filter(pod_id.eq(pod.id))
                .first::<QueueModel>(&*db)
                .unwrap();

            // Check if the given module name maps to a module type.
            if ModuleTypes::from_str(entry.module_name.as_str()).is_err() {
                return bad_request()
                    .message(format!("No such module type `{}`", entry.module_name).as_str());
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
                .get_result::<QueueEntryModel>(&*db)
                .expect("Failed to update user.");

            created().message("Queue entry added.").data(json!(&new_queue_entry))
        }
    }
}
