use diesel::prelude::*;
use rocket_contrib::{JSON, SerdeError};

use helpers::db::DB;
use helpers::request::validate_json;
use responses::{APIResponse, ok, internal_server_error};
use validation::pod::PodSettingsSerializer;

use models::pod::Pod;
use models::user::User;


/// Endpoint for setting different values for your pod
#[post("/settings", data = "<pod_data>", format = "application/json")]
pub fn settings(
    pod_data: Result<JSON<PodSettingsSerializer>, SerdeError>,
    current_user: User,
    db: DB,
) -> Result<APIResponse, APIResponse> {

    let mut pod = current_user.get_pod(&db);
    let pod_settings = validate_json(pod_data)?;

    if let Some(ref name) = pod_settings.name {
        pod.name = name.clone();
    }
    pod.save_changes::<Pod>(&*db).or(
        Err(internal_server_error()),
    )?;

    Ok(ok().data(json!(&pod)))
}
