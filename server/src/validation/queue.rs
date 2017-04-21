use validator::Validate;


#[derive(Debug, Validate, Deserialize)]
pub struct QueueAddResearchSerializer {
    pub research_name: String,
    pub level: i32,
}


#[derive(Debug, Validate, Deserialize)]
pub struct QueueAddModuleSerializer {
    pub module_name: String,
    pub position_x: Option<i32>,
    pub position_y: Option<i32>,
    pub level: i32,
}
