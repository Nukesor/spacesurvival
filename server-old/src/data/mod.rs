pub mod helper;
pub mod types;
pub mod modules;
pub mod components;
pub mod researches;

use self::types::ResearchTypes;

/// A trait we need to build generic functions for module and research dependecy checking.
///
/// It ensures that the inserted HashMap contains elements with a vector of researches.
pub trait HasDependencies {
    fn get_dependencies(&self) -> &Vec<(ResearchTypes, i32)>;
}


/// Helper function to set default value of research dependencies
fn dependencies_default() -> Vec<(ResearchTypes, i32)> {
    let dependencies: Vec<(ResearchTypes, i32)> = Vec::new();
    dependencies
}
