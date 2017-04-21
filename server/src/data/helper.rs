use data::types::*;
use data::researches::{
    Research,
    Level as ResearchLevel,
    RESEARCH_LIST
};


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
