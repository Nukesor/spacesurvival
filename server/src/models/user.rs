use uuid::Uuid;
use chrono::{DateTime, Utc, Duration};
use argon2rs::argon2i_simple;
use ring::constant_time::verify_slices_are_equal;

use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;

use helpers::db::DB;
use responses::{APIResponse, internal_server_error};

use models::pod::Pod;
use models::queue::Queue;
use models::resource::Resource;

use schema::users;
use schema::pods::dsl as pods_dsl;
use schema::queues::dsl as queues_dsl;


#[derive(Debug, Serialize, Deserialize, Identifiable, Queryable, Associations, AsChangeset)]
pub struct User {
    pub id: Uuid,
    pub nickname: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: Vec<u8>,
    pub current_auth_token: Option<String>,

    pub last_action: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
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

    /// Hash `password` using argon2 and return it.
    pub fn make_password_hash(password: &str) -> Vec<u8> {
        argon2i_simple(password, "loginsalt").to_vec()
    }

    /// Verify that `candidate_password` matches the stored password.
    pub fn verify_password(&self, candidate_password: &str) -> bool {
        let candidate_hash = argon2i_simple(candidate_password, "loginsalt").to_vec();
        self.password_hash == candidate_hash
    }

    /// Generate an auth token and save it to the `current_auth_token` column.
    pub fn generate_auth_token(&mut self, conn: &PgConnection) -> Result<String, APIResponse> {
        let new_auth_token = Uuid::new_v4().hyphenated().to_string();
        self.current_auth_token = Some(new_auth_token.clone());
        self.last_action = Some(Utc::now());
        self.save_changes::<User>(conn).or(
            Err(internal_server_error()),
        )?;
        Ok(format!(
            "{}:{}",
            self.id.hyphenated().to_string(),
            new_auth_token
        ))
    }

    /// Return whether or not the user has a valid auth token.
    pub fn has_valid_auth_token(&self, auth_token_timeout: Duration) -> bool {
        let latest_valid_date = Utc::now() - auth_token_timeout;
        if let Some(last_action) = self.last_action {
            if self.current_auth_token.is_some() {
                last_action > latest_valid_date
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn get_curret_auth_token(self) -> Result<String, APIResponse> {
        let token = self.current_auth_token.ok_or(internal_server_error())?;
        Ok(format!("{}:{}", self.id.hyphenated().to_string(), token))
    }

    /// Get a `User` from a login token.
    ///
    /// A login token has this format:
    ///     <user uuid>:<auth token>
    pub fn get_user_from_login_token(token: &str, db: &PgConnection) -> Option<User> {
        use schema::users::dsl::*;

        let v: Vec<&str> = token.split(':').collect();
        // Invalid token has been sent
        if v.len() < 2 {
            return None;
        }
        let (user_id, auth_token) = (Uuid::parse_str(v[0]).unwrap_or(Uuid::nil()), v[1]);

        let user = users.filter(id.eq(user_id)).first::<User>(&*db).optional();
        if let Ok(Some(u)) = user {
            if let Some(token) = u.current_auth_token.clone() {
                if verify_slices_are_equal(token.as_bytes(), auth_token.as_bytes()).is_ok() {
                    return Some(u);
                }
            }
        }
        return None;
    }
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub nickname: String,
    pub email: String,
    pub password_hash: Vec<u8>,
}

#[derive(AsChangeset)]
#[table_name = "users"]
pub struct ChangedUser {
    pub nickname: Option<String>,
    pub email: Option<String>,
    pub password_hash: Option<Vec<u8>>,
}
