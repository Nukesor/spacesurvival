use diesel::prelude::*;
use rocket_contrib::{JSON, SerdeError};

use models::user::UserModel;
use schema::users::dsl::*;
use validation::user::LoginSerializer;

use helpers::db::DB;
use responses::{APIResponse, ok, unauthorized, bad_request};


#[post("/login", data = "<user_in>", format = "application/json")]
pub fn login(user_in: Result<JSON<LoginSerializer>, SerdeError>, db: DB) -> APIResponse {

    // Return specific error if invalid JSON has been sent.
    if user_in.is_err() {
        return bad_request().message(format!("{}", user_in.err().unwrap()).as_str());
    }

    let user_in = user_in.unwrap();

    let result;
    if user_in.nickname.is_some() {
        result = users.filter(nickname.eq(user_in.nickname.as_ref().unwrap().clone()))
            .first::<UserModel>(&*db);
    }
    else if user_in.nickname.is_some() {
        result = users.filter(email.eq(user_in.email.as_ref().unwrap().clone()))
            .first::<UserModel>(&*db);
    }
    else {
        return bad_request()
            .message("Either a nickname or an email needs to be specified");
    }

    if result.is_err() {
        return unauthorized().message("Username or password incorrect.");
    }

    let user = result.unwrap();
    if !user.verify_password(user_in.password.as_str()) {
        return unauthorized().message("Username or password incorrect.");
    }

    ok().data(json!(user.generate_auth_token("loginsalt")))
}


