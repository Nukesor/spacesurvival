#[derive(Debug, Eq, Clone, Hash, PartialEq, Serialize, Deserialize)]
pub enum ResearchTypes {
    PlasmaGenerator,
    EnergyWeapons,
    MiningEfficiency,
}


#[derive(Debug, Eq, Clone, Hash, PartialEq, Serialize, Deserialize)]
pub enum ResourceTypes {
    Iron,
    Water,
}


#[derive(Debug, Eq, Clone, Hash, PartialEq, Serialize, Deserialize)]
pub enum ModuleTypes {
    Turret,
    Generator,
}
