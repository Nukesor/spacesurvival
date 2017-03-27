use uuid::Uuid;
use validator::Validate;


#[derive(Debug, Deserialize, Validate)]
#[validate(schema(function = "validate_research_or_module"))]
pub struct QueueAddSerializer {
    pub research_id: Option<Uuid>,
    pub module_id: Option<Uuid>,
    pub level: i32,
}


#[derive(Debug, Deserialize, Validate)]
pub struct PodSettingsSerializer {
    pub name: Option<String>,
}


fn validate_research_or_module(data: &QueueAddSerializer) -> Option<(String, String)> {
    if data.research_id.is_some() && data.module_id.is_some() {
        return Some(("research_id".to_string(), "Don't provide a research and a module id".to_string()))
    }

// We don't need this as we have to do this in the route.
// The compiler will complain otherwise. But I'll let this here anyway.

//    if data.research_id.is_none() && data.module_id.is_none() {
//        return Some((String::from("research_id"), String::from("Either a module id or a research id needs to be specified.")))
//    }
//
    None
}
