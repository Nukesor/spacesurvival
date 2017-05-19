use uuid::Uuid;
use validator::Validate;


#[derive(Debug, Validate, Deserialize)]
pub struct ResearchSerializer {
    pub research_name: String,
    pub level: i32,
}


#[derive(Debug, Validate, Deserialize)]
pub struct NewModuleSerializer {
    pub module_name: String,
    pub position_x: Option<i32>,
    pub position_y: Option<i32>,
    pub level: i32,
}


#[derive(Debug, Validate, Deserialize)]
pub struct UpgradeModuleSerializer {
    pub module_id: Uuid,
    pub position_x: Option<i32>,
    pub position_y: Option<i32>,
    pub level: i32,
}
