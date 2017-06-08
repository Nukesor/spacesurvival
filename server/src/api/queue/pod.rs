use diesel::prelude::*;

use helpers::db::DB;
use responses::{APIResponse, ok};

use models::user::User;
use models::queue::QueueEntry;

use schema::queue_entries::dsl as queue_entry_dsl;


/// The user needs to be logged in to access this route!
///
/// This route returns the list of the currently running entries in the queue.
#[get("/pod")]
pub fn pod_queue_entries(current_user: User, db: DB) -> Result<APIResponse, APIResponse> {

    let (_, queue) = current_user.get_pod_and_queue(&db);

    let queue_entries = queue_entry_dsl::queue_entries
        .filter(queue_entry_dsl::queue_id.eq(queue.id))
        .order(queue_entry_dsl::created_at.asc())
        .get_results::<QueueEntry>(&*db)
        .or(Err(ok().message("Queue is empty.")))?;

    Ok(ok().data(json!(&queue_entries)))
}
