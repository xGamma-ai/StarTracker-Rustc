use actix_web::{HttpResponse, Responder, http::StatusCode, post};

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[post("/register_user")]
async fn register_user(req_body: String) -> impl Responder {
    HttpResponse::Ok().body("body")
}
