pub mod types;
pub mod modules;
pub mod components;
pub mod researches;


use data::types::*;

#[derive(Debug, PartialEq, Deserialize)]
pub struct Resource {
    pub name: ResourceTypes,
    pub amount: i32,
}
