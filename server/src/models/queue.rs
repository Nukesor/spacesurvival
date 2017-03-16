use uuid::Uuid;
use chrono::NaiveDateTime;

use schema::{queues,queue_entries};


#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct QueueModel {
    pub slots: i32,
    pub id: Uuid,
    pub pod_id: Option<Uuid>,
    pub base_id: Option<Uuid>,
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
    pub research_id: Option<Uuid>,
    pub module_id: Option<Uuid>,
//    pub duration: PgInterval,
    pub created_at: NaiveDateTime,
}


#[derive(Insertable)]
#[table_name="queue_entries"]
pub struct NewQueueEntry {
//    pub duration: PgInterval,
    pub queue_id: Uuid,
    pub research_id: Option<Uuid>,
    pub module_id: Option<Uuid>,
}
