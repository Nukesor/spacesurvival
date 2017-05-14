pub mod helper;
pub mod types;
pub mod modules;
pub mod components;
pub mod researches;

use self::types::ResearchTypes;

/// A trait we need to build generic functions for module and research dependecy checking.  
/// It ensures that the inserted HashMap contains elements with a vector of researches.
pub trait HasDependencies {
    fn get_dependencies(&self) -> Option<&Vec<(ResearchTypes, i32)>>;
}
