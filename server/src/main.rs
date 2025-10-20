use actix_web::{get, App, HttpServer, Responder, middleware::Logger};
use actix_files::Files;
use env_logger;

fn files() -> Files {
    Files::new("/", "../static")
        .index_file("index.html")
        .prefer_utf8(true)
}

fn args() -> (&'static str, u16) {
    let port = std::env::args().nth(1)
        .and_then(|x| x.parse().ok())
        .unwrap_or(8080);
    ("0.0.0.0", port)
}

#[get("/ping")]
async fn ping() -> impl Responder {"Ok"}

#[actix_web::main] 
async fn main() -> std::io::Result<()> {
    let (addr, port) = args();
    println!("prepare to listen on {addr}:{port}");

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    HttpServer::new(|| { 
        App::new()
            .wrap(Logger::default()
                .log_target("access_log"))
            .service(ping)
            .service(files())
    })
        .bind((addr, port))?
        .run()
        .await
}