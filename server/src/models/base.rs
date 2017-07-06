use uuid::Uuid;
use chrono::{DateTime, Utc};

use schema::bases;


#[derive(Debug, Serialize, Deserialize, Identifiable, Queryable, Associations)]
pub struct Base {
    pub name: String,
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}


#[derive(Insertable)]
#[table_name = "bases"]
pub struct NewBase {
    pub name: String,
}
