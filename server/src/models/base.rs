use uuid::Uuid;
use chrono::{DateTime, UTC};

use schema::{bases, pods, modules, researches, resources};


#[derive(Debug, Serialize, Deserialize, Identifiable, Queryable, Associations)]
#[has_many(pods)]
#[has_many(modules)]
#[has_many(researches)]
#[has_many(resources)]
pub struct Base {
    pub name: String,
    pub id: Uuid,
    pub created_at: DateTime<UTC>,
    pub updated_at: DateTime<UTC>,
}


#[derive(Insertable)]
#[table_name="bases"]
pub struct NewBase {
    pub name: String,
}
