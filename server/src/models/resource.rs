use diesel;
use uuid::Uuid;
use diesel::prelude::*;

use data::types::*;
use helpers::db::DB;

use schema::resources;
use schema::resources::dsl as resources_dsl;


#[derive(Debug, Serialize, Deserialize, Identifiable, Queryable, Associations)]
#[belongs_to(pods)]
#[belongs_to(bases)]
pub struct Resource {
    pub name: String,
    pub amount: i64,
    pub max_amount: i64,

    pub id: Uuid,
    pub pod_id: Option<Uuid>,
    pub base_id: Option<Uuid>,
}


impl Resource {
    /// This function checks if there are enough resources for a given set
    /// of costs from a research or model entry.  
    /// It also subtracts and updates the resources the specified value from the database.
    ///
    /// The first parameter is a vector of resources which represent the costs.  
    /// The second parameter is a vector of all `Resource` database models from a pod or a base.  
    pub fn check_resources(costs: &Option<Vec<(ResourceTypes, i64)>>,
                           resources: Vec<Resource>,
                           db: &DB)
                           -> bool {
        match costs.as_ref() {
            // There are no costs for this module/research
            None => return true,
            // There are some costs
            Some(costs) => {
                for &(ref resource_type, amount) in costs {
                    // Try to get the correct entry from existing resources.
                    let existing = resources
                        .iter()
                        .filter(|x| x.name == resource_type.to_string())
                        .next();
                    match existing {
                        // There is no resource for this resource_type,
                        // thereby it's not enough.
                        None => return false,
                        // There is a resource for this resource type
                        // We need to check if we got enough of it.
                        Some(existing_resource) => {
                            if existing_resource.amount < amount {
                                return false;
                            }
                        }
                    }
                }
                Resource::update_resources(costs, resources, true, db);
                return true;
            }
        }
    }


    /// This function updates the resources the specified value from the database.  
    /// Depending on the third parameter the costs will either be added or subtracted.
    ///
    /// The first parameter is a vector of resources which represent the costs.  
    /// The second parameter is a vector of all `Resource` database models from a pod or a base.  
    pub fn update_resources(costs: &Vec<(ResourceTypes, i64)>,
                            resources: Vec<Resource>,
                            subtract: bool,
                            db: &DB) {

        for &(ref resource_type, amount) in costs.iter() {
            // Try to get the correct entry from existing resources.
            let resource = resources
                .iter()
                .filter(|x| x.name == resource_type.to_string())
                .next();
            match resource {
                // There is no resource for this resource_type,
                // thereby it's not enough.
                // There is a resource for this resource type
                Some(resource) => {
                    resource.update_resource(amount, subtract, db);
                }
                None => (),
            }
        }
    }


    /// This function updates a resource in the database.  
    /// Depending on the third parameter the value will either be added or subtracted.
    ///
    /// The first parameter is the queried database model.  
    /// The second parameter is the amount to be added or subtracted.  
    pub fn update_resource(&self, amount: i64, subtract: bool, db: &DB) {
        let mut new_amount: i64;
        if subtract {
            new_amount = self.amount - amount;
        } else {
            new_amount = self.amount + amount;
            if new_amount > self.max_amount {
                new_amount = self.max_amount;
            }
        }

        let updated_resource = UpdatedResource {
            amount: Some(new_amount),
            max_amount: None,
        };

        diesel::update(resources_dsl::resources.filter(resources_dsl::id.eq(self.id)))
            .set(&updated_resource)
            .get_result::<Resource>(&**db)
            .expect("Failed to update resource.");
    }
}


#[derive(Insertable)]
#[table_name="resources"]
pub struct NewResource {
    pub name: String,
    pub max_amount: i64,
    pub pod_id: Option<Uuid>,
    pub base_id: Option<Uuid>,
}


#[derive(AsChangeset)]
#[table_name="resources"]
pub struct UpdatedResource {
    pub amount: Option<i64>,
    pub max_amount: Option<i64>,
}
