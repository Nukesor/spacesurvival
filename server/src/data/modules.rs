#[derive(Debug, PartialEq, Deserialize)]
pub struct Level {
    level: i32,
    value: i32,
    radius: i32,
}


#[derive(Debug, PartialEq, Deserialize)]
pub struct Module {
    name: String,
    module_type: String,
    level: Vec<Level>,
}
