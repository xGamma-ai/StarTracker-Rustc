use actix_web::{HttpResponse, Responder, post, web};
use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper};

use crate::{
    establish_connection,
    models::{UserData, UserPasswordDetails, WriteNewUser, WriteNewUserPassword},
    schema::{password_manager, user_data},
    user_access_management::serializers::{UserLoginInfo, UserRegisterInfo},
    utils::{login_password_hasher, verify_pwd_state},
};

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
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
    println!("LOGIN REQUEST HAS BEEN HIT");
    let joined: Result<(UserData, UserPasswordDetails), diesel::result::Error> = user_data::table
        .inner_join(password_manager::table.on(password_manager::user_id.eq(user_data::id)))
        .filter(user_data::user_name.eq(&req_body.user_name))
        .select((UserData::as_select(), UserPasswordDetails::as_select()))
        .first(&mut establish_connection());
    match joined {
        Ok((user, pwd)) => {
            if verify_pwd_state(&pwd.password_hash, &pwd.salt, &req_body.user_password) {
                return HttpResponse::Ok().body("User Validated!");
            }
            println!("UserName : {} {}", user.user_name, pwd.salt);
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
