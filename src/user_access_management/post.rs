use std::{
    sync::{Arc, Mutex},
    thread,
};

use actix_web::{
    Error, HttpMessage, HttpResponse, Responder, dev::ServiceRequest, error::ErrorUnauthorized,
    post, web,
};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper};
use serde_json::json;

use crate::{
    establish_connection,
    models::{UserData, UserPasswordDetails, UserSettings, WriteNewUser, WriteNewUserPassword},
    schema::{password_manager, user_data, user_settings},
    user_access_management::{
        jwt::{UserToken, gen_jwt, verify_jwt},
        serializers::{ApiUserSettings, PostLoginDataInfo, UserLoginInfo, UserRegisterInfo},
    },
    utils::{login_password_hasher, verify_pwd_state},
};

pub async fn jwt_validate(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    if credentials.token().is_empty() {
        return Err((ErrorUnauthorized("Invalid token"), req));
    }
    let claims = verify_jwt(&credentials.token());
    match claims {
        Ok(c) => {
            req.extensions_mut().insert(c);
            return Ok(req);
        }
        Err(e) => return Err((ErrorUnauthorized(e.to_string()), req)),
    }
}

#[post("/register_user")]
async fn register_user(req_body: web::Json<UserRegisterInfo>) -> impl Responder {
    use crate::schema::user_data;
    let new_user_data = WriteNewUser {
        user_email: &req_body.user_email,
        user_name: &req_body.user_name,
    };
    let created_user = diesel::insert_into(user_data::table)
        .values(&new_user_data)
        .returning(UserData::as_returning())
        .get_result(&mut establish_connection())
        .expect("Failed to insert new user data.");

    //if new username insertion is success. We proceed to hash and save User password.
    let (out_u8, salt) = login_password_hasher(&req_body.user_password);
    let new_pwd_data = WriteNewUserPassword {
        password_hash: out_u8,
        salt: salt,
        user_id: created_user.id,
    };
    let r_json = json!({
        "json_formation_test":"TEST_JSON_VALUE"
    });
    let addi_value =
        serde_json::from_value(r_json).expect("Failed to parse the serde_json definition.");
    let user_settings = Arc::new(Mutex::new(UserSettings {
        user_id: created_user.id,
        enable_online_mode: false,
        addtional_settings: Option::Some(addi_value),
    }));
    diesel::insert_into(password_manager::table)
        .values(&new_pwd_data)
        .execute(&mut establish_connection())
        .expect("Failed to save the new hashed password");

    let user_settings_arc_clone = Arc::clone(&user_settings);
    thread::spawn(move || {
        let u_settings_insert = user_settings_arc_clone.lock().unwrap();
        diesel::insert_into(user_settings::table)
            .values(u_settings_insert.clone())
            .execute(&mut establish_connection())
            .unwrap();
    });
    HttpResponse::Ok().body(format!("New User added {}", &req_body.user_name))
}

#[post("/login")]
async fn login_user(req_body: web::Json<UserLoginInfo>) -> impl Responder {
    let joined: Result<(UserData, UserPasswordDetails, UserSettings), diesel::result::Error> =
        user_data::table
            .inner_join(password_manager::table.on(password_manager::user_id.eq(user_data::id)))
            .inner_join(user_settings::table.on(user_settings::user_id.eq(user_data::id)))
            .filter(user_data::user_email.eq(&req_body.user_email))
            .select((
                UserData::as_select(),
                UserPasswordDetails::as_select(),
                UserSettings::as_select(),
            ))
            .first(&mut establish_connection());
    match joined {
        Ok((_user, pwd, user_set)) => {
            if verify_pwd_state(&pwd.password_hash, &pwd.salt, &req_body.user_password) {
                //if user has been validated send a JWT auth token
                let user_jwt = gen_jwt(UserToken {
                    user_email: (&req_body.user_email).clone(),
                })
                .unwrap();
                return HttpResponse::Ok().json(PostLoginDataInfo {
                    jwt_token: user_jwt,
                    user_settings: ApiUserSettings {
                        addtional_settings: user_set.addtional_settings,
                        enable_online_mode: user_set.enable_online_mode,
                    },
                });
            }
        }
        Err(diesel::result::Error::NotFound) => {
            return HttpResponse::NotFound().body("user not found");
        }
        Err(_e) => {
            return HttpResponse::InternalServerError().body("failed to execute login.");
        }
    }
    return HttpResponse::InternalServerError().body("failed to execute login.");
}
