use diesel;
use diesel::prelude::*;

use chrono::UTC;

use helpers::db::DB;

use models::module::Module;
use models::research::Research;
use models::queue::QueueEntry;

use schema::modules::dsl as module_dsl;
use schema::researches::dsl as research_dsl;
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

    if let Ok(finished_entries) = finished_entries_result {
        for entry in finished_entries {
            match entry {
                QueueEntry{module_id: Some(module_id), ..} => {
                    let module = Module::get(module_id, &db).expect("QueueEntry with invalid Module.id");
                    diesel::update(module_dsl::modules.find(module.id))
                        .set(module_dsl::level.eq(module.level + 1))
                        .execute(&*db)
                        .expect("Failed to update module level.");
                }
                QueueEntry{research_id: Some(research_id), ..} => {
                    let research = Research::get(research_id, &db).expect("QueueEntry with invalid Research.id");
                    diesel::update(research_dsl::researches.find(research.id))
                        .set(research_dsl::level.eq(research.level + 1))
                        .execute(&*db)
                        .expect("Failed to update module level.");
                }
                _ => (),
            }

            diesel::delete(queue_entries_dsl::queue_entries
                    .filter(queue_entries_dsl::id.eq(entry.id)))
                .execute(&*db)
                .expect("Failed to remove queue_entry.");
        }
    }
    ok().message("Tick successful.")
}
