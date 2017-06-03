use petgraph::Graph;
use petgraph::graph::NodeIndex;
use petgraph::algo::is_cyclic_directed;

use serde_yaml::from_slice;
use std::collections::HashMap;

use data::types::*;
use data::{HasDependencies, dependencies_default};

static RESEARCH_LIST: &'static [u8] = include_bytes!("../../research_data.yml");

/// Helper function to set default value of current_level
fn current_level_default() -> i32 { 0 } 


/// This struct is only for deserializing the included `research_data.yml`.
///
/// It shouldn't be used in any other context!
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Research {
    pub name: String,
    #[serde(default = "dependencies_default")]
    pub dependencies: Vec<(ResearchTypes, i32)>,
    #[serde(default = "current_level_default")]
    pub current_level: i32,
    pub levels: Vec<Level>,
}

impl HasDependencies for Research {
    fn get_dependencies(&self) -> &Vec<(ResearchTypes, i32)> {
        self.dependencies.as_ref()
    }
}

/// This struct is only for deserializing the included `research_data.yml`.
///
/// It shouldn't be used in any other context!
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Level {
    pub level: i32,
    pub resources: Option<Vec<(ResourceTypes, i64)>>,
    pub time: i64,
}

/// This function builds a petgraph graph from the statically included research_data.yml.
///
/// After checking if petgraph is able to create a graph from it, a dependency circle check
/// will be executed.
///
/// # Panics
/// - If an unknown ResearchType is detected during graph creation.
/// - If a dependency circle is detected.
/// - If serde-yml tries to parse invalid yml.
pub fn build_research_graph() -> Graph<ResearchTypes, i32> {
    let research_list = get_research_list();
    let mut dependency_graph = Graph::<ResearchTypes, i32>::new();
    let mut nodes = HashMap::<ResearchTypes, NodeIndex>::new();
    for (research_type, _) in &research_list {
        let dependency = dependency_graph.add_node(research_type.clone());
        nodes.insert(research_type.clone(), dependency);
    }
    for (research_type, research) in research_list {
        let original_node = nodes.get(&research_type).unwrap();
        if research.dependencies.len() != 0 {
            for &(ref dependency_type, level) in research.dependencies.iter() {
                let dependency_node = nodes.get(dependency_type);
                match dependency_node {
                    Some(dependency) => {
                        dependency_graph.add_edge(*dependency, *original_node, level);
                    }
                    None => panic!("Unknown dependency {:?}", &dependency_type),
                }
            }
        }
    }
    if is_cyclic_directed(&dependency_graph) {
        panic!("Cycle in research list detected. Pls fix!");
    }
    dependency_graph
}

/// This function builds builds a HashMap from `research_data.yml`.
/// It contains: All researches, their levels, costs per level and research dependencies.
///
/// ```
/// static RESEARCH_LIST: &'static [u8] = include_bytes!("../../research_data.yml");
/// ```
///
/// # Panics
/// - If serde-yml tries to parse invalid yml.
pub fn get_research_list() -> HashMap<ResearchTypes, Research> {
    let result = from_slice::<HashMap<ResearchTypes, Research>>(RESEARCH_LIST);
    match result {
        Ok(v) => v,
        Err(e) => panic!("{:?}", e),
    }
}
