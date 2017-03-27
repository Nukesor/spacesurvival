use diesel;
use diesel::prelude::*;
use rocket_contrib::{JSON, SerdeError};

use helpers::db::DB;
use validation::pod::{PodSettingsSerializer, QueueAddSerializer};

use schema::{pods, queue_entries};
use schema::pods::dsl::*;
use schema::queues::dsl::*;

use models::pod::{PodModel, ChangedPod};
use models::user::UserModel;
use models::queue::{QueueEntryModel, QueueModel, NewQueueEntry};


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
            let current_pod = pods.filter(user_id.eq(current_user.id.clone()))
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


#[post("/queue/add", data = "<queue_entry>", format = "application/json")]
pub fn add_to_queue(queue_entry: Result<JSON<QueueAddSerializer>, SerdeError>, current_user: UserModel, db: DB) -> APIResponse {

    // Return specific error if invalid JSON has been sent.
    match queue_entry {
        Err(error) => return bad_request().message(format!("{}", error).as_str()),
        Ok(entry) =>  {
            // Get pod and queue from db
            let pod = pods.filter(user_id.eq(current_user.id.clone()))
                .first::<PodModel>(&*db)
                .unwrap();

            let queue = queues.filter(pod_id.eq(pod.id.clone()))
                .first::<QueueModel>(&*db)
                .unwrap();

            // Create a queue entry with a module or research id
            // depending which was provided in the payload
            let new_entry_model: NewQueueEntry;
            if entry.research_id.is_some() {
                new_entry_model = NewQueueEntry {
                    queue_id: queue.id.clone(),
                    module_id: None,
                    research_id: entry.research_id.clone(),
                    level: entry.level.clone(),
                };
            }
            else if entry.module_id.is_some() {
                new_entry_model = NewQueueEntry {
                    queue_id: queue.id.clone(),
                    module_id: entry.module_id.clone(),
                    research_id: None,
                    level: entry.level.clone(),
                };
            }
            else {
                return bad_request()
                    .message("Either a module or a research needs to be specified");
            }

            let new_queue_entry = diesel::insert(&new_entry_model)
                .into(queue_entries::table)
                .get_result::<QueueEntryModel>(&*db)
                .expect("Failed to update user.");

            created().message("Queue entry added.").data(json!(&new_queue_entry))
        }
    }
}
