use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
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
