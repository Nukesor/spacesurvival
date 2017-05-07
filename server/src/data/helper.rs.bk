use std::hash::Hash;
use std::collections::HashMap;

use diesel::result::Error;

use models::research::Research;
use models::resource::Resource;

use data::types::*;
use data::researches::RESEARCH_LIST;
use helpers::db::DB;


pub trait HasDependencies {
    fn get_dependencies(&self) -> Option<&Vec<(ResearchTypes, i32)>>;
}

pub fn get_research_dependency_strings(research_type: &ResearchTypes) -> Vec<String> {
    let ref research_list = RESEARCH_LIST;
    let mut dependency_strings = Vec::new();
    let ref dependency_list = research_list.get(research_type)
        .as_ref().unwrap().dependencies;
    match *dependency_list {
        None => (),
        Some(ref dependencies) => {
            for &(ref dependency, _) in dependencies {
                dependency_strings.push(dependency.to_string());
            }
        }
    }
    dependency_strings
}

/*
Generic function which accepts an Enum as type identifier.
*/
pub fn dependencies_fulfilled<T: Eq + Hash, M: HasDependencies>(
    reliant_type: &T,
    fulfilled_result: Result<Vec<Research>, Error>,
    list: &HashMap<T, M>) -> bool {
    // Get all researches required for the specified type.
    let requirement_list = list.get(reliant_type)
        .as_ref().unwrap().get_dependencies();
    match requirement_list {
        // No dependencies for this type
        None => return true,
        // Check if dependencies are fulfilleed
        Some(requirements) => {
            // Check if we got any required researches
            let fulfilled_list = match fulfilled_result {
                Ok(result) => result,
                Err(_) => return false,
            };
            for &(ref requirement, level) in requirements {
                // Try to get the correct entry from fulfilled vector
                let fulfilled = fulfilled_list.iter()
                    .filter(|x| x.name == requirement.to_string())
                    .next();
                match fulfilled {
                    // There is no research for this dependency, thereby it's not fulfilled
                    None => return false,
                    // We found an existing research, check if the level is sufficient.
                    Some(research) => {
                        if research.level < level {
                            return false
                        }
                    }
                }
            }
            return true;
        }
    }
}

/*
Generic function which accepts an Enum as type identifier.
*/
pub fn check_resources(
    costs: Option<Vec<(ResourceTypes, i64)>>,
    resources: Vec<Resource>,
    db: DB) -> bool {

    match costs {
        // There are no costs for this module/research
        None => return true,
        // There are some costs
        Some(costs) => {
            for (ref resource_type, amount) in costs {
                // Try to get the correct entry from existing resources.
                let existing = resources.iter()
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
            return true;
        }
    }
}
