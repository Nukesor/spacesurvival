use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};

use models::user::User;
use helpers::db::DB;
use responses::{APIResponse, bad_request, unauthorized, forbidden, not_found,
                internal_server_error, service_unavailable};


#[error(400)]
fn bad_request_handler() -> APIResponse {
    bad_request()
}

#[error(401)]
fn unauthorized_handler() -> APIResponse {
    unauthorized()
}

#[error(403)]
fn forbidden_handler() -> APIResponse {
    forbidden()
}

#[error(404)]
fn not_found_handler() -> APIResponse {
    not_found()
}

#[error(500)]
fn internal_server_error_handler() -> APIResponse {
    internal_server_error()
}

#[error(503)]
fn service_unavailable_handler() -> APIResponse {
    service_unavailable()
}


/// This `FromRequest` implementation for the `User` database model tries to get
/// the current user from the request `OAUTH` token.
///
/// If there is no active session for this token, we return a `401 Unauthorized`.
impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<User, ()> {
        let db = <DB as FromRequest>::from_request(request).unwrap();
        let tokens: Vec<_> = request.headers().get("Authorization").collect();
        if tokens.len() != 1 {
            return Outcome::Failure((Status::Unauthorized, ()));
        }

        let token = tokens[0];

        match User::get_user_from_auth_token(&token, "loginsalt", &*db) {
            Some(user) => Outcome::Success(user),
            None => Outcome::Failure((Status::Unauthorized, ())),
        }

    }
}
