use diesel;
use diesel::prelude::*;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use helpers::db::DB;

use schema::modules;
use schema::modules::dsl as module_dsl;


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

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}


impl Module {
    /// This function adds a new pod into the database and returns the model with the
    /// initialized data.
    pub fn get(id: Uuid, db: &DB) -> Result<Module, diesel::result::Error> {
        // Get the research from an id
        module_dsl::modules
            .filter(module_dsl::id.eq(id))
            .get_result::<Module>(&**db)
    }
}


#[derive(Insertable)]
#[table_name = "modules"]
pub struct NewModule {
    pub name: String,
    pub level: i32,
    pub stationary: bool,
    pub x_pos: Option<i32>,
    pub y_pos: Option<i32>,
    pub pod_id: Option<Uuid>,
    pub base_id: Option<Uuid>,
}