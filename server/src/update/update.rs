use diesel;
use diesel::prelude::*;
use chrono::UTC;

use helpers::db::DB;

use models::pod::Pod;
use models::module::Module;
use models::research::Research;
use models::queue::{Queue, QueueEntry};

use schema::pods::dsl as pods_dsl;
use schema::modules::dsl as module_dsl;
use schema::researches::dsl as research_dsl;
use schema::queues::dsl as queue_dsl;
use schema::queue_entries::dsl as queue_entry_dsl;


/// Update all game entities and check for finished jobs.
///
/// - Check for finished QueueEntries
/// - Update resources
/// - Compute fights
pub fn tick(db: &DB) {

    let finished_entries_result = queue_entry_dsl::queue_entries
        .filter(queue_entry_dsl::finishes_at.lt(UTC::now()))
        .get_results::<QueueEntry>(&**db);

    if let Ok(finished_entries) = finished_entries_result {
        for entry in finished_entries {
            match entry {
                QueueEntry{module_id: Some(module_id), ..} => {
                    let module = Module::get(module_id, db).expect("QueueEntry with invalid Module.id");
                    diesel::update(module_dsl::modules.find(module.id))
                        .set(module_dsl::level.eq(module.level + 1));
                    let pod = pods_dsl::pods
                        .filter(pods_dsl::user_id.eq(module.id))
                        .first::<Pod>(&**db)
                        .unwrap();
                    pod.update_resources(&db);
                }
                QueueEntry{research_id: Some(research_id), ..} => {
                    let research = Research::get(research_id, db).expect("QueueEntry with invalid Research.id");
                    diesel::update(research_dsl::researches.find(research.id))
                        .set(research_dsl::level.eq(research.level + 1))
                        .execute(&**db)
                        .expect("Failed to update module level.");
                }
                _ => (),
            }

            let queue = queue_dsl::queues
                .filter(queue_dsl::id.eq(entry.queue_id))
                .first::<Queue>(&**db)
                .unwrap();

            queue.remove_entry(entry.id, db)
        }
    }
}
