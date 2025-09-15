use actix_web::{HttpResponse, Responder, http::StatusCode, post, web};
use diesel::{RunQueryDsl, SelectableHelper};

use crate::{
    establish_connection,
    models::{UserData, WriteNewUser},
    schema::user_data,
    user_access_management::serializers::UserRegisterInfo,
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
    diesel::insert_into(user_data::table)
        .values(&new_user_data)
        .returning(UserData::as_returning())
        .get_result(&mut establish_connection())
        .expect("Failed to insert new user data.");

    HttpResponse::Ok().body(format!("New User added {}", &req_body.user_name))
}
