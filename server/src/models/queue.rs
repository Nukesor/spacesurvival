use uuid::Uuid;
use chrono::NaiveDateTime;

use schema::{queues,queue_entries};


#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct QueueModel {
    pub id: Uuid,
    pub pod_id: Option<Uuid>,
    pub base_id: Option<Uuid>,
    pub slots: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}


#[derive(Insertable)]
#[table_name="queues"]
pub struct NewQueue {
    pub slots: i32,
    pub pod_id: Option<Uuid>,
    pub base_id: Option<Uuid>,
}


#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct QueueEntryModel {
    pub id: Uuid,
    pub queue_id: Uuid,
    pub module_name: Option<String>,
    pub research_name: Option<String>,
    pub level: i32,
    pub created_at: NaiveDateTime,
}


#[derive(Insertable)]
#[table_name="queue_entries"]
pub struct NewQueueEntry {
    pub queue_id: Uuid,
    pub research_name: Option<String>,
    pub module_name: Option<String>,
    pub level: i32,
}
