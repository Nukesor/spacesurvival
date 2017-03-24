use std::collections::HashMap;

use std::fs::File;
use serde_yaml::from_reader;
use std::env;


#[derive(Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum ResearchTypes {
    PlasmaGenerator,
    EnergyWeapons
}


#[derive(Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum ResourceTypes {
    Iron,
    Water
}


#[derive(Debug, PartialEq, Deserialize)]
pub struct Level {
    pub level: i32,
    pub resources: Vec<Resource>,
}


#[derive(Debug, PartialEq, Deserialize)]
pub struct Resource {
    pub name: ResourceTypes,
    pub amount: i32,
}


#[derive(Debug, PartialEq, Deserialize)]
pub struct Research {
    pub name: ResearchTypes,
    pub dependencies: Option<Vec<ResearchTypes>>,
    pub level: Vec<Level>,
}


lazy_static! {
    pub static ref RESEARCH_LIST_2: HashMap<ResearchTypes, Research> = {
        let p = env::current_dir().unwrap();
        println!("The current directory is {}", p.display());
        let file = File::open("./server/data.yml");
        match file {
            Ok(v) => {
                let result = from_reader::<File, HashMap<ResearchTypes, Research>>(v);
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
                panic!("Panic mal. Research YAML nicht gefunden!");
            },
        }
    };
}
