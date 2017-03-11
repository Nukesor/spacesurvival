use uuid::Uuid;
use chrono::NaiveDateTime;

use schema::bases;


#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Base {
    pub name: String,
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}


#[derive(Insertable)]
#[table_name="bases"]
pub struct NewBase {
    pub name: String,
}
