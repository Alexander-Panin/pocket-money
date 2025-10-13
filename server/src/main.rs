use actix_web::{get, web, App, HttpServer, Responder};
use actix_files::Files;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let port = std::env::args().nth(1)
        .and_then(|x| x.parse().ok())
        .unwrap_or(8080);
    let addr = "0.0.0.0";
    println!("prepare to listen on {addr}:{port}");
    
    HttpServer::new(|| { 
        App::new()
            .service(
                Files::new("/", "../static")
                    .index_file("index.html")
                    .prefer_utf8(true))
            .service(greet)
    })
        .bind((addr, port))?
        .run()
        .await
}