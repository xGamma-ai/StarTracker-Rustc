pub mod models;
pub mod routes;
pub mod schema;
pub mod user_access_management;
pub mod utils;

use crate::user_access_management::*;
use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use diesel::prelude::*;
use dotenvy::dotenv;

async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("The Server is alive.")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    let host = std::env::var("HOST").expect("No host found");
    let port = std::env::var("PORT")
        .expect("No port found")
        .parse::<u16>()
        .expect("Port must be a number");
    HttpServer::new(|| {
        App::new()
            .service(echo)
            .service(verify_user)
            .service(register_user)
            .service(login_user)
            .route("/health", web::get().to(health_check))
    })
    .bind((host, port))?
    .run()
    .await
}

/*
this is the connection setup for the database of Diesel.rs
*/
pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
