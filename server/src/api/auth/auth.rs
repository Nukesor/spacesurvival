use diesel::prelude::*;
use rocket_contrib::{JSON, SerdeError};

use models::user::UserModel;
use schema::users::dsl::*;
use validation::user::LoginSerializer;

use helpers::db::DB;
use responses::{APIResponse, ok, unauthorized, bad_request};


#[post("/login", data = "<user_in>", format = "application/json")]
pub fn login(user_in: Result<JSON<LoginSerializer>, SerdeError>, db: DB) -> APIResponse {

    match user_in {
        // Return specific error if invalid JSON has been sent.
        Err(error) => return bad_request().message(format!("{}", error).as_str()),
        Ok(user_in) => {
            let mut result;
            // Check if the identifier is a nickname.
            result = users.filter(nickname.eq(&user_in.identifier))
                .first::<UserModel>(&*db);

            match result {
                // The identifier was a nickname!
                Ok(user) => {
                    if !user.verify_password(user_in.password.as_str()) {
                        return unauthorized().message("Password incorrect.");
                    }
                    return ok().data(json!(user.generate_auth_token("loginsalt")));
                },
                // Check if the identifier is an email address
                Err(_) => {
                    result = users.filter(email.eq(&user_in.identifier))
                        .first::<UserModel>(&*db);

                    match result {
                        // There is no such email or nickname.
                        Err(_) => return unauthorized().message("Nickname or email doesn't exist."),
                        // The identifier was an email!
                        Ok(user) => {
                            if !user.verify_password(user_in.password.as_str()) {
                                return unauthorized().message("Password incorrect.");
                            }
                            return ok().data(json!(user.generate_auth_token("loginsalt")));
                        },
                    }
                }
            }
        }
    }
}
