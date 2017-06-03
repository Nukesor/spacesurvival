use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use uuid::Uuid;
use chrono::{DateTime, UTC};
use argon2rs::argon2i_simple;
use jsonwebtoken::{encode, decode, Header, Algorithm};

use helpers::util;
use helpers::db::DB;

use models::pod::Pod;
use models::queue::Queue;
use models::resource::Resource;

use schema::users;
use schema::pods::dsl as pods_dsl;
use schema::queues::dsl as queues_dsl;


#[derive(Debug, Serialize, Deserialize, Identifiable, Queryable, Associations)]
pub struct User {
    pub id: Uuid,
    pub nickname: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: Vec<u8>,

    pub created_at: DateTime<UTC>,
    pub updated_at: DateTime<UTC>,
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
struct UserLoginToken {
    user_id: Uuid,
}

impl User {
    /// Create a new User and inject it into the database.
    ///
    /// # Panics
    /// Panics if there is an error while inserting the User
    pub fn new_user(nickname: String, email: String, password_hash: Vec<u8>, db: &DB) -> Self {
        // New user model for table insertion
        let new_user = NewUser {
            nickname: nickname,
            email: email,
            password_hash: password_hash,
        };

        // Insert and return user
        let user = diesel::insert(&new_user)
            .into(users::table)
            .get_result::<User>(&**db)
            .expect("Error inserting new user into database.");

        // Create new Pod with queue
        let pod = Pod::new_pod(user.nickname.clone(), user.id, db);
        Queue::new_pod_queue(pod.id, db);

        // Create resources for pod
        Resource::create_pod_resources(pod.id, db);

        user
    }

    pub fn get_pod(&self, db: &DB) -> Pod {
        // Get pod and queue from db
        pods_dsl::pods
            .filter(pods_dsl::user_id.eq(self.id))
            .first::<Pod>(&**db)
            .unwrap()
    }


    pub fn get_pod_and_queue(&self, db: &DB) -> (Pod, Queue) {
        // Get pod and queue from db
        let pod = pods_dsl::pods
            .filter(pods_dsl::user_id.eq(self.id))
            .first::<Pod>(&**db)
            .unwrap();
        let queue = queues_dsl::queues
            .filter(queues_dsl::pod_id.eq(pod.id))
            .first::<Queue>(&**db)
            .unwrap();

        (pod, queue)
    }

    pub fn make_password_hash(new_password: &str) -> Vec<u8> {
        argon2i_simple(new_password, "loginsalt").to_vec()
    }

    pub fn verify_password(&self, candidate_password: &str) -> bool {
        let candidate_hash = argon2i_simple(candidate_password, "loginsalt").to_vec();
        self.password_hash == candidate_hash
    }

    pub fn generate_auth_token(&self, salt: &str) -> String {
        let secret = util::get_secret();
        let combined_secret = secret + salt;

        encode(Header::default(),
               &UserLoginToken { user_id: self.id },
               combined_secret.as_bytes())
                .unwrap()
    }

    pub fn get_user_from_auth_token(token: &str, salt: &str, db: &PgConnection) -> Option<User> {
        use schema::users::dsl::*;

        let secret = util::get_secret();
        let combined_secret = secret + salt;

        let decrypted_token =
            decode::<UserLoginToken>(&token, combined_secret.as_bytes(), Algorithm::HS256);
        if decrypted_token.is_err() {
            return None;
        }

        let token = decrypted_token.unwrap();

        let user = users
            .filter(id.eq(token.claims.user_id))
            .first::<User>(&*db);
        if user.is_err() {
            return None;
        }

        Some(user.unwrap())
    }
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub nickname: String,
    pub email: String,
    pub password_hash: Vec<u8>,
}

#[derive(AsChangeset)]
#[table_name="users"]
pub struct ChangedUser {
    pub nickname: Option<String>,
    pub email: Option<String>,
    pub password_hash: Option<Vec<u8>>,
}
