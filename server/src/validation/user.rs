use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Debug, Validate)]
pub struct UserSerializer {
    pub id: Option<Uuid>,
    pub nickname: String,
    #[validate(email)]
    pub email: String,
    pub password: String,
}
