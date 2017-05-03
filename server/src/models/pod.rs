use uuid::Uuid;
use chrono::NaiveDateTime;

use schema::{pods, modules, researches, resources};


#[derive(Debug, Serialize, Deserialize, Identifiable, Queryable, Associations)]
#[belongs_to(users)]
#[belongs_to(bases)]
#[has_many(modules)]
#[has_many(researches)]
#[has_many(resources)]
pub struct Pod {
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


#[derive(AsChangeset)]
#[table_name="pods"]
pub struct ChangedPod {
    pub name: Option<String>,
}
