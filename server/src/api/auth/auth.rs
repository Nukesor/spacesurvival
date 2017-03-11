use std::error::Error;

use diesel;
use diesel::prelude::*;
use rocket_contrib::{JSON, SerdeError};

use models::user::{UserModel, NewUser};
use schema::users;
use schema::users::dsl::*;
use validation::user::UserSerializer;

use helpers::db::DB;
use responses::{APIResponse, ok, created, conflict, unauthorized, bad_request};


#[post("/login", data = "<user_in>", format = "application/json")]
pub fn login(user_in: Result<JSON<UserSerializer>, SerdeError>, db: DB) -> APIResponse {
    match user_in {
        Result::Err(err) => return bad_request().message(err.description()),
        _ => (),
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


#[post("/register", data = "<user_data>", format = "application/json")]
pub fn register(user_data: Result<JSON<UserSerializer>, SerdeError>, db: DB) -> APIResponse {

    match user_data {
        Result::Err(err) => return bad_request().message(err.description()),
        _ => (),
    }
    let user_data = user_data.unwrap();

    let results = users.filter(email.eq(user_data.email.clone()))
        .first::<UserModel>(&*db);
    if results.is_ok() {
        return conflict().message("User already exists.");
    }

    let new_password_hash = UserModel::make_password_hash(user_data.password.as_str());
    let new_user = NewUser {
        email: user_data.email.clone(),
        password_hash: new_password_hash,
    };

    let user = diesel::insert(&new_user)
        .into(users::table)
        .get_result::<UserModel>(&*db)
        .expect("Error saving new post");

    created().message("User created.").data(json!(&user))
}
