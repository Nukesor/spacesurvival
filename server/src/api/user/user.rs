use diesel;
use diesel::prelude::*;
use rocket_contrib::Json;

use helpers::db::DB;
use validation::user::{UserSerializer, UserSettingsSerializer};
use responses::{APIResponse, ok, created, conflict, forbidden, unauthorized,
                internal_server_error};

use models::user::{User, ChangedUser};
use schema::users::dsl::*;


#[get("/info")]
pub fn info(current_user: User) -> APIResponse {
    ok().data(json!(current_user))
}


/// Endpoint for registering a new User.
///
/// Needs a unique nickname, unique email and password.
#[post("/register", data = "<data>", format = "application/json")]
pub fn register(
    data: Json<UserSerializer>,
    db: DB,
) -> Result<APIResponse, APIResponse> {

    // Check for existing user email
    users
        .filter(email.eq(data.email.clone()))
        .first::<User>(&*db)
        .optional()
        .or(Err(conflict().message("Email already taken.")))?;

    // Check for existing user nickname
    users
        .filter(email.eq(data.email.clone()))
        .first::<User>(&*db)
        .optional()
        .or(Err(conflict().message("Nickname already taken.")))?;

    // Create new password hash
    let new_password_hash = User::make_password_hash(data.password.as_str());

    // Create new user to get uuid for pod
    let user = User::new_user(
        data.nickname.clone(),
        data.email.clone(),
        new_password_hash,
        &db,
    );

    Ok(created().data(json!(&user)))
}


#[post("/settings", data = "<data>", format = "application/json")]
pub fn settings(
    current_user: User,
    data: Json<UserSettingsSerializer>,
    db: DB,
) -> Result<APIResponse, APIResponse> {
    let mut new_password_hash: Option<Vec<u8>> = None;
    // Check if a new password is provided.
    // In case it is, we want the old password to verify the identity of the client.
    if let Some(ref new_password) = data.new_password {
        if let Some(ref old_password) = data.password {
            if !current_user.verify_password(old_password.as_str()) {
                return Err(unauthorized().message("Incorrect password."));
            }
            new_password_hash = Some(User::make_password_hash(new_password.as_str()));
        } else {
            return Err(forbidden().message(
                "The current passwords needs to be \
                                            specified, if you want to change your password.",
            ));
        }
    }

    let changed_user = ChangedUser {
        nickname: data.nickname.clone(),
        email: data.email.clone(),
        password_hash: new_password_hash,
    };

    let user = diesel::update(users.filter(id.eq(current_user.id)))
        .set(&changed_user)
        .get_result::<User>(&*db)
        .or(Err(internal_server_error()))?;

    Ok(ok().data(json!(&user)))
}
