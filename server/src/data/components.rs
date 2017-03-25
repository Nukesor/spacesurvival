#[derive(Debug, PartialEq, Deserialize)]
pub struct Shoots {
    pub rate: i32,
    pub damage: i32,
    pub range: i32,
}


#[derive(Debug, PartialEq, Deserialize)]
pub struct GeneratesEnergy {
    pub output: i32,
}
