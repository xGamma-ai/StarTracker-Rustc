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
    pub user_email: String,
    pub user_password: String,
}

#[derive(Serialize)]
pub struct PostLoginDataInfo {
    pub jwt_token: String,
    pub user_settings: ApiUserSettings,
}

#[derive(Serialize, Deserialize)]
pub struct ApiUserSettings {
    pub addtional_settings: Option<serde_json::Value>,
    pub enable_online_mode: bool,
}

#[derive(Deserialize, Debug)]
pub struct AddtionalSettingsFormat {
    pub selected_theme: Option<String>,
    pub device_os: Option<String>,
    pub device_name: Option<String>,
    pub json_formation_test: Option<String>,
}
