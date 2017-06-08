use diesel::prelude::*;
use rocket_contrib::{JSON, SerdeError};

use helpers::db::DB;
use responses::{APIResponse, ok, unauthorized, bad_request};
use validation::user::LoginSerializer;

use models::user::User;
use schema::users::dsl::*;


/// Endpoint for login.
///
/// Check if we can login with the credentials.
/// We try to get the user by searching email and nickname for the given identifier.
#[post("/login", data = "<user_data>", format = "application/json")]
pub fn login(user_data: Result<JSON<LoginSerializer>, SerdeError>, db: DB) -> Result<APIResponse, APIResponse> {

    // Return specific error if invalid JSON has been sent.
    if let Err(error) = user_data {
        return Err(bad_request().message(format!("{}", error).as_str()));
    }
    let user_login = user_data.unwrap();
    // Check if the identifier is a nickname.
    let mut user_result = users
        .filter(nickname.eq(&user_login.identifier))
        .first::<User>(&*db);

    if user_result.is_err() {
        user_result = users
            .filter(email.eq(&user_login.identifier))
            .first::<User>(&*db);
    }
    let mut user = user_result.or(Err(unauthorized().message("Nickname or email doesn't exist.")))?;

    if !user.verify_password(user_login.password.as_str()) {
        return Err(unauthorized().message("Password incorrect."));
    }
    let auth_token = user.generate_auth_token(&db)?;
    Ok(ok().data(json!(auth_token)))
}
