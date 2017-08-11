use diesel::prelude::*;
use rocket::State;
use rocket_contrib::Json;

use helpers::db::DB;
use responses::{APIResponse, ok, unauthorized};
use validation::user::LoginSerializer;
use RuntimeConfig;

use models::user::User;
use schema::users::dsl::*;


/// Endpoint for login.
///
/// Check if we can login with the credentials.
/// We try to get the user by searching email and nickname for the given identifier.
#[post("/login", data = "<user_login>", format = "application/json")]
pub fn login(
    user_login: Json<LoginSerializer>,
    rconfig: State<RuntimeConfig>,
    db: DB,
) -> Result<APIResponse, APIResponse> {

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
    let mut user = user_result.or(Err(unauthorized().message(
        "Nickname or email doesn't exist.",
    )))?;

    // Verify password
    if !user.verify_password(user_login.password.as_str()) {
        return Err(unauthorized().message("Password incorrect."));
    }

    let token = if user.has_valid_auth_token(rconfig.0) {
        user.get_curret_auth_token()?
    } else {
        user.generate_auth_token(&db)?
    };
    Ok(ok().data(json!(token)))
}
