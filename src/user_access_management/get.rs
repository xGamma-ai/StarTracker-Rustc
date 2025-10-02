use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder, get};
use diesel::prelude::*;

use crate::{
    establish_connection, models::UserData, schema::user_data, user_access_management::jwt::Claims,
};

#[get("/verify-user")]
async fn verify_user<'a>(req: HttpRequest) -> impl Responder {
    let user_struct = match req.extensions().get::<Claims>() {
        Some(c) => c.user_details.user_email.clone(),
        None => return HttpResponse::Unauthorized().body("User details not found in token."),
    };
    println!("incoming email {user_struct}");
    use self::user_data::dsl::*;
    let results = user_data
        .filter(user_email.eq(&user_struct))
        .select(UserData::as_select())
        .first(&mut establish_connection())
        .expect("Error pulling user data.");
    HttpResponse::Ok().body(format!("Hello world! {}", results.user_name))
}
