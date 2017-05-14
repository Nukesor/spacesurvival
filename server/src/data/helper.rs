use std::hash::Hash;
use std::collections::HashMap;

use diesel::result::Error;

use data::types::*;
use data::HasDependencies;
use data::researches::get_research_list;

use models::research::Research;


/// A helper function returns all research dependencies of a module or research as a `Vec<String>`.  
/// This is helpful for querying researches by their name.
pub fn get_research_dependency_strings(research_type: &ResearchTypes) -> Vec<String> {
    let ref research_list = get_research_list();
    let mut dependency_strings = Vec::new();
    let ref dependency_list = research_list
        .get(research_type)
        .as_ref()
        .unwrap()
        .dependencies;
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

/// A generic function which checks if all dependencies for a module or research are fulfilled.  
/// 
/// The first parameter is the type of the module or research that should be checked.  
/// The second parameter is a list of existing `Research` (The database model of `researches`).  
/// The third parameter accepts the respective parsed `MODULE_LIST` or a `RESEARCH_LIST`.  
pub fn dependencies_fulfilled<T: Eq + Hash, M: HasDependencies>(
    reliant_type: &T,
    fulfilled_result: Result<Vec<Research>, Error>,
    list: &HashMap<T, M>) -> bool{
    // Get all researches required for the specified type.
    let requirement_list = list.get(reliant_type).as_ref().unwrap().get_dependencies();
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
                let fulfilled = fulfilled_list
                    .iter()
                    .filter(|x| x.name == requirement.to_string())
                    .next();
                match fulfilled {
                    // There is no research for this dependency, thereby it's not fulfilled
                    None => return false,
                    // We found an existing research, check if the level is sufficient.
                    Some(research) => {
                        if research.level < level {
                            return false;
                        }
                    }
                }
            }
            return true;
        }
    }
}
