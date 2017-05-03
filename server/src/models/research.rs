use uuid::Uuid;
use chrono::NaiveDateTime;

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
