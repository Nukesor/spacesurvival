use uuid::Uuid;
use chrono::NaiveDateTime;

use schema::researches;


#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct ResearchModel {
    pub name: String,
    pub id: Uuid,
    pub pod_id: Option<Uuid>,
    pub base_id: Option<Uuid>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}


#[derive(Insertable)]
#[table_name="researches"]
pub struct NewResearch {
    pub name: String,
    pub pod_id: Option<Uuid>,
    pub base_id: Option<Uuid>,
}
