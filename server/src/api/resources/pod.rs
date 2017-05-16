use diesel::prelude::*;

use helpers::db::DB;

use models::pod::Pod;
use models::user::User;
use models::resource::Resource;

use schema::pods::dsl as pod_dsl;
use schema::resources::dsl as resources_dsl;

use responses::{APIResponse, ok};

/// The user needs to be logged in to access this route!
///
/// This route returns the list of resources of the users pod.
#[get("/pod")]
pub fn pod_resources(current_user: User, db: DB) -> APIResponse {
    let pod = pod_dsl::pods
        .filter(pod_dsl::user_id.eq(current_user.id))
        .get_result::<Pod>(&*db)
        .expect("Failed to get user pod.");

    let resources = resources_dsl::resources
    .filter(resources_dsl::pod_id.eq(pod.id))
    .get_results::<Resource>(&*db)
    .expect("Failed to get resources from pod.");

    ok().message("Pod Resources.").data(json!(&resources))
}
