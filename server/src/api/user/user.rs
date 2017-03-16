use std::error::Error;

use diesel;
use diesel::prelude::*;
use rocket_contrib::{JSON, SerdeError};

use schema::{users, pods};
use schema::users::dsl::*;
use models::user::{UserModel, NewUser};

use models::pod::{PodModel, NewPod};

use helpers::db::DB;
use validation::user::UserSerializer;

use responses::{APIResponse, ok, created, conflict, bad_request};


#[get("/info")]
pub fn info(current_user: UserModel) -> APIResponse {
    ok().data(json!(current_user.email))
}


#[post("/register", data = "<user_data>", format = "application/json")]
pub fn register(user_data: Result<JSON<UserSerializer>, SerdeError>, db: DB) -> APIResponse {

    // Return specific error if invalid JSON has been sent.
    match user_data {
        Result::Err(err) => return bad_request().message(err.description()),
        _ => (),
    }
    let user_data = user_data.unwrap();

    // Check for existing user email
    let results = users.filter(email.eq(user_data.email.clone()))
        .first::<UserModel>(&*db);
    if results.is_ok() {
        return conflict().message("User with this email already exists.");
    }

    // Check for existing nick
    let results = users.filter(nick.eq(user_data.nick.clone()))
        .first::<UserModel>(&*db);
    if results.is_ok() {
        return conflict().message("Nick already taken.");
    }

    // Create new password hash 
    let new_password_hash = UserModel::make_password_hash(user_data.password.as_str());

    // New user model for table insertion
    let new_user = NewUser {
        nick: user_data.nick.clone(),
        email: user_data.email.clone(),
        password_hash: new_password_hash,
    };

    // Insert user to get id for pod
    let user = diesel::insert(&new_user)
        .into(users::table)
        .get_result::<UserModel>(&*db)
        .expect("Error saving new user");

    // New pod
    let new_pod = NewPod {
        name: format!("{}'s Pod", user.nick.clone()),
        user_id: user.id.clone(),
    };

    diesel::insert(&new_pod)
        .into(pods::table)
        .get_result::<PodModel>(&*db)
        .expect("Error creating pod");

    created().message("User created.").data(json!(&user))
}
