use std::fs::File;
use petgraph::Graph;
use petgraph::graph::NodeIndex;
use petgraph::algo::is_cyclic_directed;

use serde_yaml::from_reader;
use std::collections::HashMap;

use data::Resource;
use data::types::*;


#[derive(Debug, PartialEq, Deserialize)]
pub struct Research {
    pub name: ResearchTypes,
    pub dependencies: Option<Vec<(ResearchTypes, i32)>>,
    pub level: Vec<Level>,
}


#[derive(Debug, PartialEq, Deserialize)]
pub struct Level {
    pub level: i32,
    pub resources: Vec<Resource>,
}

pub fn build_graph(research_list: &HashMap<ResearchTypes, Research>) -> Graph<ResearchTypes, i32> {
    let mut dependency_graph = Graph::<ResearchTypes, i32>::new();
    let mut nodes = HashMap::<ResearchTypes, NodeIndex>::new();
    for (research_type, _) in research_list {
        let dependency = dependency_graph.add_node(research_type.clone());
        nodes.insert(research_type.clone(), dependency);
    }
    for (research_type, research) in research_list {
        let original_node = nodes.get(&research_type).unwrap();
        match research.dependencies {
            Some(ref dependencies) => {
                for &(ref dependency_type, level) in dependencies.iter() {
                    let dependency_node = nodes.get(dependency_type);
                    match dependency_node {
                        Some(dependency) => {
                            dependency_graph.add_edge(*dependency, *original_node, level);
                        },
                        None => panic!("Unknown dependency {:?}", &dependency_type)
                    }
                }
            },
            None => (),
        }
    }
    if is_cyclic_directed(&dependency_graph) {
        panic!("Cycle in research list detected. Pls fix!");
    }
    dependency_graph
}

lazy_static! {
    pub static ref RESEARCH_LIST: HashMap<ResearchTypes, Research> = {
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
