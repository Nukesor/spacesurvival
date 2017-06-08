use diesel::prelude::*;
use rocket::State;
use rocket_contrib::{JSON, SerdeError};

use helpers::db::DB;
use helpers::request::validate_json;
use responses::{APIResponse, ok, unauthorized, internal_server_error};
use validation::user::LoginSerializer;
use RuntimeConfig;

use models::user::User;
use schema::users::dsl::*;


/// Endpoint for login.
///
/// Check if we can login with the credentials.
/// We try to get the user by searching email and nickname for the given identifier.
#[post("/login", data = "<user_data>", format = "application/json")]
pub fn login(user_data: Result<JSON<LoginSerializer>, SerdeError>,
             rconfig: State<RuntimeConfig>,
             db: DB) -> Result<APIResponse, APIResponse> {

    let user_login = validate_json(user_data)?;

    // Check if the identifier is a nickname.
    let mut user_result = users
        .filter(nickname.eq(&user_login.identifier))
        .first::<User>(&*db);

    // Check if the identifier is an email adress.
    if user_result.is_err() {
        user_result = users
            .filter(email.eq(&user_login.identifier))
            .first::<User>(&*db);
    }
    let mut user = user_result.or(Err(unauthorized().message("Nickname or email doesn't exist.")))?;

    // Verify password
    if !user.verify_password(user_login.password.as_str()) {
        return Err(unauthorized().message("Password incorrect."));
    }
    
    let token = if user.has_valid_auth_token(rconfig.0) {
        user.current_auth_token.ok_or(internal_server_error())?
    } else {
        user.generate_auth_token(&db)?
    };
    Ok(ok().data(json!(token)))
}
