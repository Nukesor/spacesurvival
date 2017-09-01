use serde_yaml::from_slice;
use std::collections::HashMap;

use data::types::*;
use data::components::*;
use data::{HasDependencies, dependencies_default};

static MODULE_LIST: &'static [u8] = include_bytes!("../../module_data.yml");

/// This struct is only for deserializing the included `module_data.yml`.
/// It shouldn't be used in any other context!
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Module {
    pub display_name: String,
    #[serde(default = "dependencies_default")]
    pub dependencies: Vec<(ResearchTypes, i32)>,
    pub levels: Vec<Level>,
}


impl HasDependencies for Module {
    fn get_dependencies(&self) -> &Vec<(ResearchTypes, i32)> {
        self.dependencies.as_ref()
    }
}

fn resource_default() -> Vec<(ResourceTypes, i64)> {
    Vec::new()
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Level {
    pub level: i32,
    pub resources: Option<Vec<(ResourceTypes, i64)>>,
    pub time: i64,
    pub shoots: Option<Shoots>,
    #[serde(default = "resource_default")]
    pub generates: Vec<(ResourceTypes, i64)>,
    #[serde(default = "resource_default")]
    pub consumes: Vec<(ResourceTypes, i64)>,
}

/// This function builds builds a HashMap from `module_data.yml`.
///
/// It contains: All modules, their levels, costs per level and research dependencies.
///
/// ```
/// static MODULE_LIST: &'static [u8] = include_bytes!("../../module_data.yml");
/// ```
///
/// # Panics
/// - If serde-yml tries to parse invalid yml.
pub fn get_module_list() -> HashMap<ModuleTypes, Module> {
    let result = from_slice::<HashMap<ModuleTypes, Module>>(MODULE_LIST);
    match result {
        Ok(v) => v,
        Err(e) => panic!("{:?}", e),
    }
}
