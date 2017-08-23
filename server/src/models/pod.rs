use std::collections::HashMap;

use diesel;
use diesel::prelude::*;
use uuid::Uuid;
use chrono::{DateTime, Utc, Duration};

use data::types::*;
use data::modules::get_module_list;
use helpers::db::DB;

use models::module::Module;
use models::resource::Resource;
use models::research::Research;

use schema::pods;
use schema::modules::dsl as module_dsl;
use schema::resources::dsl as resource_dsl;
use schema::researches::dsl as research_dsl;


#[derive(Debug, Serialize, Deserialize, Identifiable, Queryable, Associations, AsChangeset)]
#[belongs_to(users)]
#[belongs_to(bases)]
pub struct Pod {
    pub name: String,
    pub id: Uuid,
    pub user_id: Uuid,
    pub base_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
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
        resource_dsl::resources
            .filter(resource_dsl::pod_id.eq(self.id))
            .get_results(&**db)
            .expect("Failed to get user resources.")
    }

    pub fn has_enough_resources(&self, costs: &Option<Vec<(ResourceTypes, i64)>>, db: &DB) -> bool {
        let pod_resources = self.get_resources(db);
        Resource::enough_resources(costs, pod_resources, db)
    }

    pub fn update_resource_production(&self, db: &DB) {
        // Create a map to save the resource production.
        let mut resources_production: HashMap<ResourceTypes, i64> = HashMap::new();
        // Get all resources and all modules
        let resources = self.get_resources(db);
        let modules_result = self.get_modules(db);
        if let Ok(modules) = modules_result {
            let module_list = get_module_list();
            for module in modules {
                // Get all modules information to compute the current resource production
                let module_type =
                    ModuleTypes::from_string(&module.name).expect("Missing module type");
                let level = &module_list
                    .get(&module_type)
                    .as_ref()
                    .expect("No module in yml for this type.")
                    .levels
                    [module.level as usize];

                // Calculate production
                for &(ref resource_ref, amount) in &level.generates {
                    let resource_type = ResourceTypes::from_string(&resource_ref.to_string())
                        .unwrap();
                    *resources_production.entry(resource_type).or_insert(0) += amount;
                }

                // Calculate consumption
                for &(ref resource_ref, amount) in &level.consumes {
                    let resource_type = ResourceTypes::from_string(&resource_ref.to_string())
                        .unwrap();
                    *resources_production.entry(resource_type).or_insert(0) -= amount;
                }
            }

            for mut resource in resources {
                let resource_type = ResourceTypes::from_string(&resource.name).unwrap();
                match resources_production.get(&resource_type) {
                    // Calculate produced amount.
                    Some(amount) => {
                        let elapsed_time: Duration =
                            Utc::now().signed_duration_since(resource.updated_at);
                        let produced_since_last_update: i64 =
                            (resource.production * elapsed_time.num_milliseconds()) / 60 / 60 /
                                1000;
                        resource.change_resource(produced_since_last_update, false, db);
                        diesel::update(resource_dsl::resources.filter(
                            resource_dsl::id.eq(resource.id),
                        )).set(resource_dsl::production.eq(amount))
                            .execute(&**db)
                            .expect("Failed to update resource production.");
                    }
                    None => (),
                }
            }
        }
    }

    pub fn update_resources(&self, db: &DB) {
        let resources = self.get_resources(db);
        for mut resource in resources {
            let elapsed_time: Duration = Utc::now().signed_duration_since(resource.updated_at);
            let produced_since_last_update: i64 =
                (resource.production * elapsed_time.num_milliseconds()) / 60 / 60 / 1000;
            resource.change_resource(produced_since_last_update, false, db);
        }
    }
}

#[derive(Insertable)]
#[table_name = "pods"]
pub struct NewPod {
    pub name: String,
    pub user_id: Uuid,
}
