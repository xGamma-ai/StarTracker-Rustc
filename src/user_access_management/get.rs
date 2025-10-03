use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder, get};
use diesel::prelude::*;

use crate::{
    establish_connection, models::UserData, schema::user_data, user_access_management::jwt::Claims,
    utils::fetch_user_details,
};

#[get("/verify-user")]
async fn verify_user<'a>(req: HttpRequest) -> impl Responder {
    let user_struct = fetch_user_details(req);
    use self::user_data::dsl::*;
    let results = user_data
        .filter(user_email.eq(&user_struct.user_email))
        .select(UserData::as_select())
        .first(&mut establish_connection())
        .expect("Error pulling user data.");
    HttpResponse::Ok().body(format!("Hello world! {}", results.user_email))
}
