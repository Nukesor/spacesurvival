use diesel;
use diesel::prelude::*;

use validator::Validate;
use rocket_contrib::{JSON, SerdeError};

use helpers::db::DB;
use validation::user::{UserSerializer, UserSettingsSerializer};

use schema::{users, pods, queues};
use schema::users::dsl::*;

use models::user::{UserModel, NewUser, ChangedUser};
use models::pod::{PodModel, NewPod};
use models::queue::{QueueModel, NewQueue};

use responses::{
    APIResponse, 
    ok,
    created,
    conflict,
    bad_request,
    forbidden,
    unauthorized,
};


#[get("/info")]
pub fn info(current_user: UserModel) -> APIResponse {
    ok().data(json!(current_user))
}


#[post("/register", data = "<user_data>", format = "application/json")]
pub fn register(user_data: Result<JSON<UserSerializer>, SerdeError>, db: DB) -> APIResponse {

    match user_data {
        // Return specific error if invalid JSON has been sent.
        Err(error) => return bad_request().message(format!("{}", error).as_str()),
        Ok(data) =>  {
            let validation = data.validate();
            match validation {
                Err(error) => {
                    let hashmap = error.inner();
                    if hashmap.contains_key("email") {
                        return bad_request().message("Invalid email provided")
                    }
                    if hashmap.contains_key("nickname") {
                        return bad_request().message("Nickname length has to be
                                                     between 1 and 120 characters.")
                    }
                },
                Ok(_) => (),
            }

             // Check for existing user email
            let results = users.filter(email.eq(data.email.clone()))
                .first::<UserModel>(&*db);
            if results.is_ok() {
                return conflict().message("Nickname already taken.");
            }
             // Create new password hash 
            let new_password_hash = UserModel::make_password_hash(data.password.as_str());

            // New user model for table insertion
            let new_user = NewUser {
                nickname: data.nickname.clone(),
                email: data.email.clone(),
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

            return created().message("User created.").data(json!(&user));
        }
    }
}


#[post("/settings", data = "<user_data>", format = "application/json")]
pub fn settings(current_user: UserModel, user_data: Result<JSON<UserSettingsSerializer>, SerdeError>, db: DB) -> APIResponse {

    // Return specific error if invalid JSON has been sent.
    match user_data {
        Err(error) => return bad_request().message(format!("{}", error).as_str()),
        Ok(data) =>  {
            let mut new_password_hash: Option<String> = None;
            // Check if a new password is provided.
            // In case it is, we want the old password to verify the identity of the client.
            match data.new_password.as_ref() {
                Some(new_password) => {
                    match data.password.as_ref() {
                        Some(old_password) => {
                            if !current_user.verify_password(old_password.as_str()) {
                                return unauthorized().message("Incorrect password.");
                            }
                            new_password_hash = Some(UserModel::make_password_hash(new_password.as_str()))
                        }
                        None => return forbidden().message("The current passwords needs to be \
                                    specified, if you want to change your password."),
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
                .get_result::<UserModel>(&*db)
                .expect("Failed to update user.");

            ok().message("User data changed.").data(json!(&user))
        }
    }
}
