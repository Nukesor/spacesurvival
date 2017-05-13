use serde_yaml::from_slice;
use std::collections::HashMap;

use data::types::*;
use data::components::*;
use data::helper::HasDependencies;

static MODULE_LIST: &'static [u8] = include_bytes!("../../module_data.yml");

#[derive(Debug, PartialEq, Deserialize)]
pub struct Module {
    pub name: ModuleTypes,
    pub dependencies: Option<Vec<(ResearchTypes, i32)>>,
    pub level: Vec<Level>,
}


impl HasDependencies for Module {
    fn get_dependencies(&self) -> Option<&Vec<(ResearchTypes, i32)>> {
        self.dependencies.as_ref()
    }
}


#[derive(Debug, PartialEq, Deserialize)]
pub struct Level {
    pub level: i32,
    pub resources: Vec<(ResourceTypes, i64)>,
    pub shoots: Option<Shoots>,
    pub generates_energy: Option<GeneratesEnergy>,
}


pub fn get_module_list() -> HashMap<ModuleTypes, Module> {
    let result = from_slice::<HashMap<ModuleTypes, Module>>(MODULE_LIST);
    match result {
        Ok(v) => v,
        Err(e) => panic!("{:?}", e),
    }
}
