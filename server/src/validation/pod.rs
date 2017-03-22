use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
#[validate(schema(function = "validate_research_or_module"))]
pub struct QueueAddSerializer {
    pub research_id: Option<Uuid>,
    pub module_id: Option<Uuid>,
    pub level: i32,
}


fn validate_research_or_module(data: &QueueAddSerializer) -> Option<(String, String)> {
    if data.research_id.is_some() && data.module_id.is_some() {
        return Some(("research_id".to_string(), "Don't provide a research and a module id".to_string()))
    }
    None
}
