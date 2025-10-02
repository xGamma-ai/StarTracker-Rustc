use chrono::{Duration, Utc};
use dotenvy::dotenv;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserToken {
    pub user_email: String,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub user_details: UserToken,
    exp: i64,
}

pub fn gen_jwt(user_details: UserToken) -> Result<String, String> {
    dotenv().ok();
    let jwt_secret_key = std::env::var("JWT_GEN_KEY").expect("Failed to secure the key.");
    let jwt_exp_time =
        std::env::var("expiration_mins_jwt").expect("Failed to detect ENV for exp time.");
    let token = encode(
        &Header::default(),
        &Claims {
            exp: (Utc::now()
                + Duration::minutes(
                    jwt_exp_time
                        .parse::<i64>()
                        .expect("Failed to parse into int"),
                ))
            .timestamp(),
            user_details: user_details,
        },
        &EncodingKey::from_secret(jwt_secret_key.as_bytes()),
    )
    .map_err(|e| e.to_string());
    return token;
}

pub fn verify_jwt(token: &str) -> Result<Claims, String> {
    dotenv().ok();
    let jwt_secret_key = std::env::var("JWT_GEN_KEY").expect("Failed to secure the key.");
    let extract_token = decode(
        token,
        &DecodingKey::from_secret(jwt_secret_key.as_bytes()),
        &Validation::default(),
    );
    match extract_token {
        Ok(token_data) => Ok(token_data.claims),
        Err(e) => Err(e.to_string()),
    }
}
