use diesel;
use diesel::prelude::*;
use rocket_contrib::{JSON, SerdeError};

use helpers::db::DB;
use helpers::request::validate_json;
use responses::{APIResponse, ok, internal_server_error};
use validation::pod::PodSettingsSerializer;

use models::pod::{Pod, ChangedPod};
use models::user::User;

use schema::pods::dsl as pods_dsl;


/// Endpoint for setting different values for your pod
#[post("/settings", data = "<pod_data>", format = "application/json")]
pub fn settings(pod_data: Result<JSON<PodSettingsSerializer>, SerdeError>,
                current_user: User,
                db: DB)
                -> Result<APIResponse, APIResponse> {

    let pod_settings = validate_json(pod_data)?;
    // Get current pod
    let current_pod = current_user.get_pod(&db);

    // Create changed pod model and push it to the DB
    let changed_pod = ChangedPod { name: pod_settings.name.clone() };
    let pod = diesel::update(pods_dsl::pods.filter(pods_dsl::id.eq(current_pod.id)))
        .set(&changed_pod)
        .get_result::<Pod>(&*db)
        .or(Err(internal_server_error()))?;

    Ok(ok().data(json!(&pod)))
}
