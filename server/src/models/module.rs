use uuid::Uuid;
use chrono::NaiveDateTime;

use schema::modules;


#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct ModuleModel {
    pub name: String,
    pub id: Uuid,
    pub pod_id: Uuid,
    pub base_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}


#[derive(Insertable)]
#[table_name="modules"]
pub struct NewModule {
    pub name: String,
    pub pod_id: Uuid,
    pub base_id: Uuid,
}
