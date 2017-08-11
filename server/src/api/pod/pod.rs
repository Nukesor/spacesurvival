use diesel::prelude::*;
use rocket_contrib::Json;

use helpers::db::DB;
use responses::{APIResponse, ok, internal_server_error};
use validation::pod::PodSettingsSerializer;

use models::pod::Pod;
use models::user::User;


/// Endpoint for setting different values for your pod
#[post("/settings", data = "<pod_settings>", format = "application/json")]
pub fn settings(
    pod_settings: Json<PodSettingsSerializer>,
    current_user: User,
    db: DB,
) -> Result<APIResponse, APIResponse> {

    let mut pod = current_user.get_pod(&db);

    if let Some(ref name) = pod_settings.name {
        pod.name = name.clone();
    }
    pod.save_changes::<Pod>(&*db).or(
        Err(internal_server_error()),
    )?;

    Ok(ok().data(json!(&pod)))
}
