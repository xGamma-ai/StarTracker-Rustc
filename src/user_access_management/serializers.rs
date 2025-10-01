//serializer to for user registry

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct UserRegisterInfo {
    pub user_name: String,
    pub user_email: String,
    pub user_password: String,
}

#[derive(Deserialize)]
pub struct UserLoginInfo {
    pub user_name: String,
    pub user_password: String,
}

#[derive(Serialize)]
pub struct PostLoginDataInfo {
    pub jwt_token: String,
}
