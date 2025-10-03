use crate::{establish_connection, models::UserData, user_access_management::jwt::Claims};
use actix_web::{HttpMessage, HttpRequest};
use diesel::{
    ExpressionMethods, RunQueryDsl, SelectableHelper,
    query_dsl::methods::{FilterDsl, SelectDsl},
};

pub fn fetch_user_details(http_req: HttpRequest) -> UserData {
    use crate::schema::user_data;
    let user_email = http_req
        .extensions()
        .get::<Claims>()
        .expect("Failed to extract claims data")
        .user_details
        .user_email
        .to_string();
    let user_details_from_token = user_data::table
        .filter(user_data::user_email.eq(&user_email))
        .select(UserData::as_select())
        .first(&mut establish_connection())
        .expect("Failed to fetch user details!");
    return user_details_from_token;
}
