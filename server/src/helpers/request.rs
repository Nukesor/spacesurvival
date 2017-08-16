use rocket_contrib::{Json, SerdeError};
use responses::{APIResponse, bad_request};

pub fn validate_json<T>(request_data: Result<Json<T>, SerdeError>) -> Result<Json<T>, APIResponse> {
    match request_data {
        Ok(value) => return Ok(value),
        Err(error) => return Err(bad_request().message(format!("{}", error).as_str())),
    };
}
