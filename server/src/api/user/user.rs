use diesel;
use diesel::prelude::*;

use std::error::Error;
use rocket_contrib::{JSON, SerdeError};

use helpers::db::DB;
use validation::user::UserSerializer;

use schema::{users, pods, queues};
use schema::users::dsl::*;

use models::user::{UserModel, NewUser};
use models::pod::{PodModel, NewPod};
use models::queue::{QueueModel, NewQueue};

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

    // Check for existing nickname
    let results = users.filter(nickname.eq(user_data.nickname.clone()))
        .first::<UserModel>(&*db);
    if results.is_ok() {
        return conflict().message("Nickname already taken.");
    }

    // Create new password hash 
    let new_password_hash = UserModel::make_password_hash(user_data.password.as_str());

    // New user model for table insertion
    let new_user = NewUser {
        nickname: user_data.nickname.clone(),
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
        name: format!("{}'s Pod", user.nickname.clone()),
        user_id: user.id.clone(),
    };

    let pod = diesel::insert(&new_pod)
        .into(pods::table)
        .get_result::<PodModel>(&*db)
        .expect("Error creating pod");

    let new_queue = NewQueue {
        slots: 2,
        pod_id: Some(pod.id.clone()),
        base_id: None,
    };

    diesel::insert(&new_queue)
        .into(queues::table)
        .get_result::<QueueModel>(&*db)
        .expect("Error creating pod's queue");

    created().message("User created.").data(json!(&user))
}
