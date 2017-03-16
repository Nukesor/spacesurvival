use uuid::Uuid;

use schema::resources;


#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct ResourceModel {
    pub id: Uuid,
    pub name: String,
    pub max_amount: i64,
    pub pod_id: Option<Uuid>,
    pub base_id: Option<Uuid>,
}


#[derive(Insertable)]
#[table_name="resources"]
pub struct NewResource {
    pub name: String,
    pub max_amount: i64,
    pub pod_id: Option<Uuid>,
    pub base_id: Option<Uuid>,
}
