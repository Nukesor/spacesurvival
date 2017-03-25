use std::env;
use std::fs::File;
use serde_yaml::from_reader;
use std::collections::HashMap;

use data::Resource;
use data::types::*;
use data::components::*;


#[derive(Debug, PartialEq, Deserialize)]
pub struct Module {
    pub name: ModuleTypes,
    pub dependencies: Option<Vec<ResearchTypes>>,
    pub level: Vec<Level>,
}


#[derive(Debug, PartialEq, Deserialize)]
pub struct Level {
    pub level: i32,
    pub resources: Vec<Resource>,
    pub shoots: Option<Shoots>,
    pub generates_energy: Option<GeneratesEnergy>,
}


lazy_static! {
    pub static ref MODULE_LIST: HashMap<ModuleTypes, Module> = {
        let p = env::current_dir().unwrap();
        println!("The current directory is {}", p.display());
        let file = File::open("./server/module_data.yml");
        match file {
            Ok(v) => {
                let result = from_reader::<File, HashMap<ModuleTypes, Module>>(v);
                match result {
                    Ok(v) => {
                        return v;
                    },
                    Err(e) => {
                        panic!("{:?}", e);
                    },
                }
            },
            Err(_) => {
                panic!("Panic mal. Module YAML nicht gefunden!");
            },
        }
    };
}
