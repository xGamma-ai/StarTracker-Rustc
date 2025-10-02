use actix_web::{
    Error, HttpMessage, HttpResponse, Responder, dev::ServiceRequest, error::ErrorUnauthorized,
    post, web,
};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper};

use crate::{
    establish_connection,
    models::{UserData, UserPasswordDetails, WriteNewUser, WriteNewUserPassword},
    schema::{password_manager, user_data},
    user_access_management::{
        jwt::{UserToken, gen_jwt, verify_jwt},
        serializers::{PostLoginDataInfo, UserLoginInfo, UserRegisterInfo},
    },
    utils::{login_password_hasher, verify_pwd_state},
};

pub async fn jwt_validate(
    mut req: ServiceRequest,
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
    diesel::insert_into(password_manager::table)
        .values(&new_pwd_data)
        .execute(&mut establish_connection())
        .expect("Failed to save the new hashed password");

    HttpResponse::Ok().body(format!("New User added {}", &req_body.user_name))
}

#[post("/login")]
async fn login_user(req_body: web::Json<UserLoginInfo>) -> impl Responder {
    let joined: Result<(UserData, UserPasswordDetails), diesel::result::Error> = user_data::table
        .inner_join(password_manager::table.on(password_manager::user_id.eq(user_data::id)))
        .filter(user_data::user_email.eq(&req_body.user_email))
        .select((UserData::as_select(), UserPasswordDetails::as_select()))
        .first(&mut establish_connection());
    match joined {
        Ok((user, pwd)) => {
            if verify_pwd_state(&pwd.password_hash, &pwd.salt, &req_body.user_password) {
                //if user has been validated send a JWT auth token
                let user_jwt = gen_jwt(UserToken {
                    user_email: (&req_body.user_email).clone(),
                })
                .unwrap();
                return HttpResponse::Ok().json(PostLoginDataInfo {
                    jwt_token: user_jwt,
                });
            }
        }
        Err(diesel::result::Error::NotFound) => {
            return HttpResponse::NotFound().body("user not found");
        }
        Err(e) => {
            return HttpResponse::InternalServerError().body("failed to execute login.");
        }
    }
    return HttpResponse::InternalServerError().body("failed to execute login.");
}
