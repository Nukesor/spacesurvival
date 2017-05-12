use diesel;
use diesel::prelude::*;

use data::types::*;
use helpers::db::DB;

use models::resource::{Resource, UpdatedResource};

use schema::resources::dsl as resources_dsl;

/*
Generic function which accepts an Enum as type identifier.
*/
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
            update_resources(resources, costs, true, db);
            return true;
        }
    }
}


pub fn update_resources(resources: Vec<Resource>,
                        costs: &Vec<(ResourceTypes, i64)>,
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
                update_resource(resource, amount, subtract, db);
            }
            None => (),
        }
    }
}

pub fn update_resource(resource: &Resource, amount: i64, subtract: bool, db: &DB) {
    let mut new_amount: i64;
    if subtract {
        new_amount = resource.amount - amount;
    } else {
        new_amount = resource.amount + amount;
        if new_amount > resource.max_amount {
            new_amount = resource.max_amount;
        }
    }
    let updated_resource = UpdatedResource {
        amount: Some(new_amount),
        max_amount: None,
    };

    diesel::update(resources_dsl::resources.filter(resources_dsl::name.eq(resource.name.clone())))
        .set(&updated_resource)
        .get_result::<Resource>(&**db)
        .expect("Failed to update resource.");
}
