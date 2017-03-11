use models::user::UserModel;
use responses::{APIResponse, ok};


#[get("/info")]
pub fn info(current_user: UserModel) -> APIResponse {
    ok().data(json!(current_user.email))
}
