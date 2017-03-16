use uuid::Uuid;
use chrono::NaiveDateTime;

use schema::pods;


#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct PodModel {
    pub name: String,
    pub id: Uuid,
    pub user_id: Uuid,
    pub base_id: Option<Uuid>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}


#[derive(Insertable)]
#[table_name="pods"]
pub struct NewPod {
    pub name: String,
    pub user_id: Uuid,
}
