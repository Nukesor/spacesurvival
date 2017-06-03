use diesel;
use diesel::prelude::*;

use uuid::Uuid;
use chrono::{DateTime, UTC};


use models::resource::Resource;

use schema::{pods, modules, researches, resources};
use schema::resources::dsl as resources_dsl;

use helpers::db::DB;


#[derive(Debug, Serialize, Deserialize, Identifiable, Queryable, Associations)]
#[belongs_to(users)]
#[belongs_to(bases)]
#[has_many(modules)]
#[has_many(researches)]
#[has_many(resources)]
pub struct Pod {
    pub name: String,
    pub id: Uuid,
    pub user_id: Uuid,
    pub base_id: Option<Uuid>,
    pub created_at: DateTime<UTC>,
    pub updated_at: DateTime<UTC>,
}


impl Pod {
    pub fn new_pod(name: String, user_id: Uuid, db: &DB) -> Self {
        // New pod
        let new_pod = NewPod {
            name: format!("{}'s Pod", name),
            user_id: user_id,
        };

        diesel::insert(&new_pod)
            .into(pods::table)
            .get_result::<Pod>(&**db)
            .expect("Error inserting new pod into database.")
    }

    pub fn get_resources(&self, db: &DB) -> Vec<Resource> {
        resources_dsl::resources
            .filter(resources_dsl::pod_id.eq(self.id))
            .get_results(&**db)
            .expect("Failed to get user resources.")
    }
}

#[derive(Insertable)]
#[table_name="pods"]
pub struct NewPod {
    pub name: String,
    pub user_id: Uuid,
}


#[derive(AsChangeset)]
#[table_name="pods"]
pub struct ChangedPod {
    pub name: Option<String>,
}
