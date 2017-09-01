use validator::Validate;


#[allow(unused_mut)]
#[derive(Debug, Validate, Deserialize)]
pub struct PodSettingsSerializer {
    pub name: Option<String>,
}
