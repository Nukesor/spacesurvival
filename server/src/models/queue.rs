use diesel;
use diesel::prelude::*;

use uuid::Uuid;
use chrono::{DateTime, UTC};

use schema::{queues, queue_entries};
use helpers::db::DB;


#[derive(Debug, Serialize, Deserialize, Identifiable, Queryable, Associations)]
#[belongs_to(pods)]
#[belongs_to(bases)]
#[has_many(queue_entries)]
pub struct Queue {
    pub id: Uuid,
    pub pod_id: Option<Uuid>,
    pub base_id: Option<Uuid>,
    pub slots: i32,
    pub created_at: DateTime<UTC>,
    pub updated_at: DateTime<UTC>,
}


impl Queue {
    pub fn new_pod_queue(pod_id: Uuid, db: &DB) -> Self {
        let new_queue = NewQueue {
            slots: 2,
            pod_id: Some(pod_id),
            base_id: None,
        };

        diesel::insert(&new_queue)
            .into(queues::table)
            .get_result::<Queue>(&**db)
            .expect("Error inserting new pod queue into database.")
    }
}

#[derive(Insertable)]
#[table_name="queues"]
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
    pub finishes_at: DateTime<UTC>,
    pub created_at: DateTime<UTC>,
}


#[derive(Insertable)]
#[table_name="queue_entries"]
pub struct NewQueueEntry {
    pub queue_id: Uuid,
    pub research_id: Option<Uuid>,
    pub research_name: Option<String>,
    pub module_name: Option<String>,
    pub module_id: Option<Uuid>,
    pub level: i32,
    pub finishes_at: DateTime<UTC>,
}
