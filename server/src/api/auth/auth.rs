use diesel::prelude::*;
use rocket_contrib::{JSON, SerdeError};

use models::user::UserModel;
use schema::users::dsl::*;
use validation::user::UserSerializer;

use helpers::db::DB;
use responses::{APIResponse, ok, unauthorized, bad_request};


#[post("/login", data = "<user_in>", format = "application/json")]
pub fn login(user_in: Result<JSON<UserSerializer>, SerdeError>, db: DB) -> APIResponse {

    // Return specific error if invalid JSON has been sent.
    if user_in.is_err() {
        return bad_request().message(format!("{}", user_in.err().unwrap()).as_str());
    }

    let user_in = user_in.unwrap();

    let results = users.filter(email.eq(user_in.email.clone()))
        .first::<UserModel>(&*db);

    if results.is_err() {
        return unauthorized().message("Username or password incorrect.");
    }

    let user = results.unwrap();
    if !user.verify_password(user_in.password.as_str()) {
        return unauthorized().message("Username or password incorrect.");
    }

    ok().data(json!(user.generate_auth_token("loginsalt")))
}


