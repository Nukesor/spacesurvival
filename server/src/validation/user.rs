use validator::Validate;

#[derive(Debug, Validate, Deserialize)]
pub struct UserSerializer {
    #[validate(length(max = "120", message = "Nicknme has to have less than 120 characters"))]
    pub nickname: String,
    #[validate(email(message = "Invalid email"))]
    pub email: String,
    #[validate(length(
            min = "5", max = "120",
            message = "Passwords has to have between 5 and 120 characters"
    ))]
    #[serde(skip_serializing)]
    pub password: String,
}


#[derive(Debug, Validate, Deserialize)]
pub struct LoginSerializer {
    pub identifier: String,
    pub password: String,
}

#[derive(Debug, Validate, Deserialize)]
pub struct UserSettingsSerializer {
    #[validate(length(max = "120", message = "Nicknme has to have less than 120 characters"))]
    pub nickname: Option<String>,
    #[validate(email(message = "Invalid email"))]
    pub email: Option<String>,
    pub password: Option<String>,
    #[validate(length(
            min = "5", max = "120",
            message = "Passwords has to have between 5 and 120 characters"
    ))]
    pub new_password: Option<String>,
}
