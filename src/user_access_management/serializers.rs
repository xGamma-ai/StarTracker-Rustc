//serializer to for user registry

use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserRegisterInfo {
    pub user_name: String,
    pub user_email: String,
    pub user_password: String,
}
