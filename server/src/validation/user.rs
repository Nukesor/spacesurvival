use validator::Validate;

#[derive(Debug, Validate, Deserialize)]
pub struct UserSerializer {
    #[validate(length(min = "1", max = "120"))]
    pub nickname: String,
    #[validate(email)]
    pub email: String,
    pub password: String,
}


#[allow(unused_mut)]
#[derive(Debug, Validate, Deserialize)]
pub struct LoginSerializer {
    pub identifier: String,
    pub password: String,
}

#[derive(Debug, Validate, Deserialize)]
pub struct UserSettingsSerializer {
    #[validate(length(min = "1", max = "120"))]
    pub nickname: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    pub password: Option<String>,
    pub new_password: Option<String>,
}
