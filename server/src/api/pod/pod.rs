use diesel;
use diesel::prelude::*;
use rocket_contrib::{JSON, SerdeError};

use helpers::db::DB;
use validation::pod::PodSettingsSerializer;


use schema::pods::dsl as pods_dsl;

use models::pod::{Pod, ChangedPod};
use models::user::User;


use responses::{APIResponse, bad_request, ok};


/// Endpoint for setting different values for your pod
#[post("/settings", data = "<pod_settings>", format = "application/json")]
pub fn settings(pod_settings: Result<JSON<PodSettingsSerializer>, SerdeError>,
                current_user: User,
                db: DB)
                -> APIResponse {

    match pod_settings {
        // Return specific error if invalid JSON has been sent.
        Err(error) => return bad_request().message(format!("{}", error).as_str()),
        Ok(settings) => {
            // Get current pod
            let current_pod = current_user.get_pod(&db);

            // Create changed pod model and push it to the DB
            let changed_pod = ChangedPod { name: settings.name.clone() };
            let pod = diesel::update(pods_dsl::pods.filter(pods_dsl::id.eq(current_pod.id)))
                .set(&changed_pod)
                .get_result::<Pod>(&*db)
                .expect("Failed to update pod.");

            ok().message("Pod data changed.").data(json!(&pod))
        }
    }
}
