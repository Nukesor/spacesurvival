use diesel::prelude::*;

use helpers::db::DB;
use responses::{APIResponse, ok, internal_server_error};

use models::pod::Pod;
use models::user::User;
use models::resource::Resource;

use schema::pods::dsl as pod_dsl;
use schema::resources::dsl as resource_dsl;


/// The user needs to be logged in to access this route!
///
/// This route returns the list of resources of the users pod.
#[get("/pod")]
pub fn pod_resources(current_user: User, db: DB) -> Result<APIResponse, APIResponse> {
    let pod = pod_dsl::pods
        .filter(pod_dsl::user_id.eq(current_user.id))
        .get_result::<Pod>(&*db)
        .or(Err(internal_server_error()))?;

    let resources = resource_dsl::resources
        .filter(resource_dsl::pod_id.eq(pod.id))
        .get_results::<Resource>(&*db)
        .or(Err(internal_server_error()))?;

    Ok(ok().data(json!(&resources)))
}
