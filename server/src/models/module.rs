use uuid::Uuid;
use chrono::NaiveDateTime;

use schema::modules;


#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct ModuleModel {
    pub name: String,
    pub id: Uuid,
    pub stationary: bool,
    pub x_pos: Option<i32>,
    pub y_pos: Option<i32>,
    pub pod_id: Option<Uuid>,
    pub base_id: Option<Uuid>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}


#[derive(Insertable)]
#[table_name="modules"]
pub struct NewModule {
    pub name: String,
    pub stationary: bool,
    pub x_pos: Option<i32>,
    pub y_pos: Option<i32>,
    pub pod_id: Option<Uuid>,
    pub base_id: Option<Uuid>,
}
