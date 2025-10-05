use actix_web::{get, web, App, HttpServer, Responder};
use actix_files::Files;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| { 
        App::new()
            .service(
                Files::new("/", "../static")
                    .index_file("index.html")
                    .prefer_utf8(true))
            .service(greet)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}