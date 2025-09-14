use actix_web::{HttpResponse, Responder, get};
use diesel::prelude::*;

use crate::{establish_connection, models::UserData, schema::user_data};

#[get("/verify-user")]
async fn verify_user() -> impl Responder {
    use self::user_data::dsl::*;
    let results = user_data
        .filter(id.eq(1))
        .limit(5)
        .select(UserData::as_select())
        .load(&mut establish_connection())
        .expect("Error pulling user data.");
    HttpResponse::Ok().body(format!("Hello world! {}", results[0].user_name))
}
