use uuid::Uuid;
use chrono::{DateTime, UTC};

use schema::researches;


#[derive(Debug, Serialize, Deserialize, Identifiable, Queryable, Associations)]
#[table_name = "researches"]
#[belongs_to(pods)]
#[belongs_to(bases)]
pub struct Research {
    pub name: String,
    pub level: i32,
    pub id: Uuid,
    pub pod_id: Option<Uuid>,
    pub base_id: Option<Uuid>,
    pub created_at: DateTime<UTC>,
    pub updated_at: DateTime<UTC>,
}


#[derive(Insertable)]
#[table_name="researches"]
pub struct NewResearch {
    pub name: String,
    pub pod_id: Option<Uuid>,
    pub base_id: Option<Uuid>,
}
