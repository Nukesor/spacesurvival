use rocket_contrib::{JSON, SerdeError};
use responses::{APIResponse, bad_request};

pub fn validate_json<T>(request_data: Result<JSON<T>, SerdeError>) -> Result<JSON<T>, APIResponse> {
    match request_data {
        Ok(value) => return Ok(value),
        Err(error) => return Err(bad_request().message(format!("{}", error).as_str())),
    };
}
