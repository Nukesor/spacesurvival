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


#[derive(Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum ModuleTypes {
    Turret,
    Generator
}
