use validator::Validate;

#[derive(Deserialize, Debug, Validate)]
pub struct UserSerializer {
    #[validate(length(min = "1", max = "120"))]
    pub nickname: String,
    #[validate(email)]
    pub email: String,
    pub password: String,
}


#[derive(Deserialize, Debug, Validate)]
pub struct LoginSerializer {
    #[validate(length(min = "1", max = "120"))]
    pub nickname: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    pub password: String,
}

#[derive(Deserialize, Debug, Validate)]
pub struct UserSettingsSerializer {
    pub nickname: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    pub password: Option<String>,
    pub new_password: Option<String>,
}
