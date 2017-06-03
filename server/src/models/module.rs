use uuid::Uuid;
use chrono::{DateTime, UTC};

use schema::modules;


#[derive(Debug, Serialize, Deserialize, Identifiable, Queryable, Associations)]
#[belongs_to(pods)]
#[belongs_to(bases)]
pub struct Module {
    pub name: String,
    pub level: i32,
    pub stationary: bool,
    pub x_pos: Option<i32>,
    pub y_pos: Option<i32>,

    pub id: Uuid,
    pub pod_id: Option<Uuid>,
    pub base_id: Option<Uuid>,

    pub created_at: DateTime<UTC>,
    pub updated_at: DateTime<UTC>,
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
