use diesel;
use diesel::prelude::*;

use validator::Validate;
use rocket_contrib::{JSON, SerdeError};

use helpers::db::DB;
use validation::user::{UserSerializer, UserSettingsSerializer};

use schema::users::dsl::*;

use models::user::{User, ChangedUser};

use responses::{APIResponse, ok, created, conflict, bad_request, forbidden, unauthorized};


#[get("/info")]
pub fn info(current_user: User) -> APIResponse {
    ok().data(json!(current_user))
}


/// Endpoint for registering a new User.
///
/// Needs a unique nickname, unique email and password.
#[post("/register", data = "<user_data>", format = "application/json")]
pub fn register(user_data: Result<JSON<UserSerializer>, SerdeError>, db: DB) -> APIResponse {

    match user_data {
        // Return specific error if invalid JSON has been sent.
        Err(error) => return bad_request().message(format!("{}", error).as_str()),
        Ok(data) => {
            let validation = data.validate();
            match validation {
                Err(error) => {
                    let hashmap = error.inner();
                    if hashmap.contains_key("email") {
                        return bad_request().message("Invalid email provided");
                    }
                    if hashmap.contains_key("nickname") {
                        return bad_request().message("Nickname length has to be
                                                     between 1 and 120 characters.");
                    }
                }
                Ok(_) => (),
            }

            // Check for existing user email
            let results = users
                .filter(email.eq(data.email.clone()))
                .first::<User>(&*db);
            if results.is_ok() {
                return conflict().message("Nickname already taken.");
            }
            // Create new password hash
            let new_password_hash = User::make_password_hash(data.password.as_str());

            // Create new user to get uuid for pod
            let user = User::new_user(data.nickname.clone(),
                                      data.email.clone(),
                                      new_password_hash,
                                      &db);

            return created().message("User created.").data(json!(&user));
        }
    }
}


#[post("/settings", data = "<user_data>", format = "application/json")]
pub fn settings(current_user: User,
                user_data: Result<JSON<UserSettingsSerializer>, SerdeError>,
                db: DB)
                -> APIResponse {

    // Return specific error if invalid JSON has been sent.
    match user_data {
        Err(error) => return bad_request().message(format!("{}", error).as_str()),
        Ok(data) => {
            let mut new_password_hash: Option<Vec<u8>> = None;
            // Check if a new password is provided.
            // In case it is, we want the old password to verify the identity of the client.
            match data.new_password.as_ref() {
                Some(new_password) => {
                    match data.password.as_ref() {
                        Some(old_password) => {
                            if !current_user.verify_password(old_password.as_str()) {
                                return unauthorized().message("Incorrect password.");
                            }
                            new_password_hash = Some(User::make_password_hash(new_password
                                                                                  .as_str()))
                        }
                        None => {
                            return forbidden().message("The current passwords needs to be \
                                    specified, if you want to change your password.")
                        }
                    }
                }
                None => (),
            };

            let changed_user = ChangedUser {
                nickname: data.nickname.clone(),
                email: data.email.clone(),
                password_hash: new_password_hash,
            };

            let user = diesel::update(users.filter(id.eq(current_user.id)))
                .set(&changed_user)
                .get_result::<User>(&*db)
                .expect("Failed to update user.");

            ok().message("User data changed.").data(json!(&user))
        }
    }
}
