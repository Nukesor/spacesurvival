use uuid::Uuid;

use schema::resources;


#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Resource {
    pub id: Uuid,
    pub name: String,
    pub max_amount: i64,
    pub pod_id: Uuid,
    pub base_id: Uuid,
}


#[derive(Insertable)]
#[table_name="resources"]
pub struct NewResource {
    pub name: String,
    pub max_amount: i64,
    pub pod_id: Uuid,
    pub base_id: Uuid,
}
