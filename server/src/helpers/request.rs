use std::collections::HashMap;

use validator::Validate;
use rocket_contrib::{Json, SerdeError};

use responses::{APIResponse, bad_request};

pub fn validate_json<T: Validate>(request_data: Result<Json<T>, SerdeError>) -> Result<Json<T>, APIResponse> {
    // Check for JSON decoding error
    match request_data {
        // Json successfully decoded, now checking for validation errors
        Ok(data) => {
            // Check for validation Error
            match data.validate() {
                // Validation errors found. Concatenate error messages.
                Err(validation_errors) => {
                    let mut formatted_errors = HashMap::new();
                    let default_message = "No explicit error Message";
                    for (name, errors) in validation_errors.inner() {
                        let mut error_string = String::new();
                        for error in errors.iter() {
                            match error.message {
                                Some(ref message) => {
                                    error_string.push_str(message.as_ref());
                                },
                                None => error_string.push_str(default_message)
                            }
                            error_string.push('\n');
                        }
                        formatted_errors.insert(name, error_string);
                    }
                    return Err(bad_request().errors(formatted_errors));
                },
                // No validation error found, return data
                Ok(_) => return Ok(data),
            }
        }
        // Json decoding error, return error message
        Err(json_error) => return Err(bad_request().message(format!("{}", json_error).as_str())),
    }
}
