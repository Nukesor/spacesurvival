use diesel;
use diesel::prelude::*;
use uuid::Uuid;
use chrono::{DateTime, UTC};

use helpers::db::DB;

use schema::researches;
use schema::researches::dsl as research_dsl;


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

impl Research {
    /// This function adds a new pod into the database and returns the model with the
    /// initialized data.
    pub fn get(id: Uuid, db: &DB) -> Result<Research, diesel::result::Error> {
      // Get the research from an id
      research_dsl::researches
          .filter(research_dsl::id.eq(id))
          .get_result::<Research>(&**db)
    }
}


#[derive(Insertable)]
#[table_name="researches"]
pub struct NewResearch {
    pub name: String,
    pub pod_id: Option<Uuid>,
    pub base_id: Option<Uuid>,
}
