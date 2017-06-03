use diesel;
use diesel::prelude::*;
use uuid::Uuid;
use chrono::{DateTime, UTC};

use data::modules::get_module_list;
use helpers::db::DB;

use models::module::Module;
use models::resource::Resource;
use models::research::Research;

use schema::{pods, modules, researches, resources};
use schema::modules::dsl as module_dsl;
use schema::resources::dsl as resources_dsl;
use schema::researches::dsl as research_dsl;


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

    pub fn get_module(&self, id: Uuid, db: &DB) -> Result<Module, diesel::result::Error> {
      // Get the research from an id
      module_dsl::modules
          .filter(module_dsl::id.eq(id))
          .filter(module_dsl::pod_id.eq(self.id))
          .get_result::<Module>(&**db)
    }

    pub fn get_modules(&self, db: &DB) -> Result<Vec<Module>, diesel::result::Error> {
        module_dsl::modules
            .filter(module_dsl::pod_id.eq(self.id))
            .get_results::<Module>(&**db)
    }

    pub fn get_researches(&self, db: &DB) -> Vec<Research> {
        research_dsl::researches
            .filter(research_dsl::pod_id.eq(self.id))
            .get_results::<Research>(&**db)
            .unwrap()
    }

    pub fn get_research(&self, name: String, db: &DB) -> Result<Research, diesel::result::Error> {
        // Get the research from an id
        research_dsl::researches
            .filter(research_dsl::pod_id.eq(self.id))
            .filter(research_dsl::name.eq(name))
            .get_result::<Research>(&**db)
    }

    pub fn get_resources(&self, db: &DB) -> Vec<Resource> {
        resources_dsl::resources
            .filter(resources_dsl::pod_id.eq(self.id))
            .get_results(&**db)
            .expect("Failed to get user resources.")
    }

    pub fn update_resources(&self, db: &DB) {
        let resources = self.get_resources(db);
        let modules = self.get_modules(db);
        let module_list = get_module_list();
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
