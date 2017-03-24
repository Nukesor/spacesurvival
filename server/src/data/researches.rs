use std::env;
use std::fs::File;
use serde_yaml::from_reader;
use std::collections::HashMap;

use data::Resource;
use data::types::*;


#[derive(Debug, PartialEq, Deserialize)]
pub struct Research {
    pub name: ResearchTypes,
    pub dependencies: Option<Vec<ResearchTypes>>,
    pub level: Vec<Level>,
}


#[derive(Debug, PartialEq, Deserialize)]
pub struct Level {
    pub level: i32,
    pub resources: Vec<Resource>,
}


lazy_static! {
    pub static ref RESEARCH_LIST: HashMap<ResearchTypes, Research> = {
        let p = env::current_dir().unwrap();
        println!("The current directory is {}", p.display());
        let file = File::open("./server/research_data.yml");
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
