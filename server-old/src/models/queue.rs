use diesel;
use diesel::prelude::*;
use uuid::Uuid;
use chrono::{DateTime, Utc, Duration};

use helpers::db::DB;

use schema::{queues, queue_entries};
use schema::queue_entries::dsl as queue_entry_dsl;


#[derive(Debug, Serialize, Deserialize, Identifiable, Queryable, Associations)]
#[belongs_to(pods)]
#[belongs_to(bases)]
pub struct Queue {
    pub id: Uuid,
    pub pod_id: Option<Uuid>,
    pub base_id: Option<Uuid>,
    pub slots: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}


impl Queue {
    pub fn new_pod_queue(pod_id: Uuid, db: &DB) -> Self {
        let new_queue = NewQueue {
            slots: 5,
            pod_id: Some(pod_id),
            base_id: None,
        };

        diesel::insert(&new_queue)
            .into(queues::table)
            .get_result::<Queue>(&**db)
            .expect("Error inserting new pod queue into database.")
    }

    /// Remove an entry to the queue and update the queue.
    pub fn remove_entry(&self, id: Uuid, db: &DB) {
        // Remove queue_entry from database
        diesel::delete(
            queue_entry_dsl::queue_entries
                .filter(queue_entry_dsl::id.eq(id))
                .filter(queue_entry_dsl::queue_id.eq(self.id)),
        ).execute(&**db)
            .expect("Failed to remove queue_entry.");
        self.update_entries(db);
    }


    /// Remove an entry to the queue and update the queue.
    pub fn add_entry(&self, entry: NewQueueEntry, db: &DB) {
        // Remove queue_entry from database
        diesel::insert(&entry)
            .into(queue_entries::table)
            .execute(&**db)
            .expect("Failed to create queue entry.");

        self.update_entries(db);
    }

    pub fn update_entries(&self, db: &DB) {
        let queue_entry_result = queue_entry_dsl::queue_entries
            .filter(queue_entry_dsl::queue_id.eq(self.id))
            .order(queue_entry_dsl::created_at.asc())
            .first::<QueueEntry>(&**db);

        if let Ok(entry) = queue_entry_result {
            if entry.finishes_at.is_none() {
                let finishes_at = Utc::now() + Duration::seconds(entry.duration);
                diesel::update(queue_entry_dsl::queue_entries.filter(
                    queue_entry_dsl::id.eq(
                        entry.id,
                    ),
                )).set(queue_entry_dsl::finishes_at.eq(finishes_at))
                    .execute(&**db)
                    .expect("Failed to update queue entries.");
            }
        }
    }
}


#[derive(Insertable)]
#[table_name = "queues"]
pub struct NewQueue {
    pub slots: i32,
    pub pod_id: Option<Uuid>,
    pub base_id: Option<Uuid>,
}


#[derive(Debug, Serialize, Deserialize, Queryable, Associations)]
#[belongs_to(queues)]
pub struct QueueEntry {
    pub id: Uuid,
    pub queue_id: Uuid,
    pub module_id: Option<Uuid>,
    pub module_name: Option<String>,
    pub research_id: Option<Uuid>,
    pub research_name: Option<String>,
    pub level: i32,
    pub duration: i64,
    pub finishes_at: Option<DateTime<Utc>>,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}


#[derive(Insertable)]
#[table_name = "queue_entries"]
pub struct NewQueueEntry {
    pub queue_id: Uuid,
    pub research_id: Option<Uuid>,
    pub research_name: Option<String>,
    pub module_name: Option<String>,
    pub module_id: Option<Uuid>,
    pub level: i32,
    pub duration: i64,
}
