use diesel::prelude::*;

use helpers::db::DB;

use models::pod::Pod;
use models::user::User;
use models::queue::{Queue, QueueEntry};

use schema::pods::dsl as pod_dsl;
use schema::queues::dsl as queues_dsl;
use schema::queue_entries::dsl as queue_entry_dsl;

use responses::{APIResponse, ok};

/// The user needs to be logged in to access this route!
///
/// This route returns the list of the currently running entries in the queue.
#[get("/pod")]
pub fn pod_queue_entries(current_user: User, db: DB) -> APIResponse {

    let pod = pod_dsl::pods
        .filter(pod_dsl::user_id.eq(current_user.id))
        .get_result::<Pod>(&*db)
        .expect("Failed to get user pod.");

    let queue = queues_dsl::queues
    .filter(queues_dsl::pod_id.eq(pod.id))
    .first::<Queue>(&*db)
    .unwrap();

    let queue_entry_result = queue_entry_dsl::queue_entries
        .filter(queue_entry_dsl::queue_id.eq(queue.id))
        .get_results::<QueueEntry>(&*db);

    if queue_entry_result.is_ok() {
        let queue_entries = queue_entry_result.unwrap();
        ok().message("Research data.").data(json!(&queue_entries))
    }
    else {
        ok().message("Queue is empty.")
    }
}
