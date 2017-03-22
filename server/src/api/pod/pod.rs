use diesel;
use diesel::prelude::*;
use rocket_contrib::{JSON, SerdeError};

use helpers::db::DB;
use validation::pod::QueueAddSerializer;

use schema::{queue_entries};
use schema::pods::dsl::*;
use schema::queues::dsl::*;

use models::pod::PodModel;
use models::user::UserModel;
use models::queue::{QueueEntryModel, QueueModel, NewQueueEntry};

use std::error::Error;

use responses::{
    APIResponse,
    created,
    bad_request,
};


#[post("/queue/add", data = "<queue_entry>", format = "application/json")]
pub fn add_to_queue(queue_entry: Result<JSON<QueueAddSerializer>, SerdeError>, current_user: UserModel, db: DB) -> APIResponse {

    // Return specific error if invalid JSON has been sent.
    match queue_entry {
        Result::Err(err) => return bad_request().message(err.description()),
        _ => (),
    }
    let queue_entry = queue_entry.unwrap();

    let pod = pods.filter(user_id.eq(current_user.id.clone()))
        .first::<PodModel>(&*db)
        .unwrap();


    let queue = queues.filter(pod_id.eq(pod.id.clone()))
        .first::<QueueModel>(&*db)
        .unwrap();

    // TODO: Check proper validation error handling (look into the response or generated error)
    // TODO: Research tree representation
    let new_queue_entry: NewQueueEntry;
    if queue_entry.research_id.is_some() {
        new_queue_entry = NewQueueEntry {
            queue_id: queue.id.clone(),
            module_id: None,
            research_id: queue_entry.research_id.clone(),
            level: queue_entry.level.clone(),
        };
    }
    else if queue_entry.module_id.is_some() {
        new_queue_entry = NewQueueEntry {
            queue_id: queue.id.clone(),
            module_id: queue_entry.module_id.clone(),
            research_id: None,
            level: queue_entry.level.clone(),
        };
    }
    else {
        return bad_request()
            .message("Either a module or a research needs to be specified");
    }


    let queue_entry = diesel::insert(&new_queue_entry)
        .into(queue_entries::table)
        .get_result::<QueueEntryModel>(&*db)
        .expect("Failed to update user.");

    created().message("Queue entry added.").data(json!(&queue_entry))
}
