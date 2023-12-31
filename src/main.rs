
use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};
use actix_web::http::header::ContentType;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(greet)
    })
        .bind(("127.0.0.1", 3000))?
        .run()
        .await
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", name)
}

