use std::ops::Deref;
use rocket::http::Status;
use rocket::{Request, State, Outcome};
use rocket::request::{self, FromRequest};
use diesel::pg::PgConnection;
use r2d2;
use r2d2_diesel::ConnectionManager;
use std::env;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

/// Generate a new r2d2 Pool for connection management in rocket sessions.
/// This object will be handed to the rocket `.manage()` managed state handler.
pub fn init_db_pool() -> Pool {
    let config = r2d2::Config::default();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::new(config, manager).expect("Failed to create pool.")
}


/// A helper struct to pass the `PooledConnection` from r2d2 to the request function.
/// It implements the `Deref` trait to easily get the `PgConnection` by using `*`
pub struct DB(r2d2::PooledConnection<ConnectionManager<PgConnection>>);

impl Deref for DB {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


/// The implementation of `FromRequest` for `DB`.
/// This function fetches the pool from the managed state of rust and gets a new
/// `PgConnection` from the `ConnectionManager`.
/// This connection will then be used during the request and closed afterwards.
impl<'a, 'r> FromRequest<'a, 'r> for DB {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<DB, ()> {
        let pool = match <State<Pool> as FromRequest>::from_request(request) {
            Outcome::Success(pool) => pool,
            Outcome::Failure(e) => return Outcome::Failure(e),
            Outcome::Forward(_) => return Outcome::Forward(()),
        };

        match pool.get() {
            Ok(conn) => Outcome::Success(DB(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}
