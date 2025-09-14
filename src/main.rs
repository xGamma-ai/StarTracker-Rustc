pub mod models;
pub mod schema;

use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};
use diesel::prelude::*;
use dotenvy::dotenv;

use crate::models::UserData;

#[get("/")]
async fn hello() -> impl Responder {
    use self::schema::user_data::dsl::*;
    let results = user_data
        .filter(id.eq(1))
        .limit(5)
        .select(UserData::as_select())
        .load(&mut establish_connection())
        .expect("Error pulling userData");
    HttpResponse::Ok().body(format!("Hello world! {}", results[0].user_name))
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

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
            .service(hello)
            .service(echo)
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
