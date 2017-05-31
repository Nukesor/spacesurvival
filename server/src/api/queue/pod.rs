use diesel::prelude::*;

use helpers::db::DB;

use models::user::User;
use models::queue::QueueEntry;

use schema::queue_entries::dsl as queue_entry_dsl;

use responses::{APIResponse, ok};

/// The user needs to be logged in to access this route!
///
/// This route returns the list of the currently running entries in the queue.
#[get("/pod")]
pub fn pod_queue_entries(current_user: User, db: DB) -> APIResponse {

    let (_, queue) = current_user.get_pod_and_queue(&db);

    let queue_entry_result = queue_entry_dsl::queue_entries
        .filter(queue_entry_dsl::queue_id.eq(queue.id))
        .get_results::<QueueEntry>(&*db);

    if queue_entry_result.is_ok() {
        let queue_entries = queue_entry_result.unwrap();
        ok().message("Research data.").data(json!(&queue_entries))
    } else {
        ok().message("Queue is empty.")
    }
}
