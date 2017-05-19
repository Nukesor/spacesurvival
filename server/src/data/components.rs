#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Shoots {
    pub rate: i32,
    pub damage: i32,
    pub range: i32,
}


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GeneratesEnergy {
    pub output: i32,
}
