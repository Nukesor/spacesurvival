use diesel::prelude::*;
use chrono;::prelude::*;

use helpers::db::DB;

use models::queue::QueueEntry;
use schema::queue_entries::dsl as queue_entries_dsl;

use responses::{APIResponse, ok};

/// A generic route, which updates all game entities and checks
/// for finished jobs. It does:
/// - Check for finished QueueEntries
/// - Update resources
/// - Compute fights
#[get("/")]
pub fn tick(db: DB) -> APIResponse {

    let finished_entries_result = queue_entries_dsl::queue_entries
        .filter(queue_entries_dsl::finishes_at.lt(UTC::now()))
        .get_results::<QueueEntry>(&*db);

    if let Some(finished_entries) = finfinished_entries_result {
        for entry in finished_entries {
            if entry.module_id.is_some() {
                
            } else if entry.research_name.is_some() {
                
            }
        }
    }
    ok().message("Queue entries updated.")
}
