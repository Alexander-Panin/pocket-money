use actix_web::{get, App, HttpServer, Responder, middleware::Logger};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod, SslAcceptorBuilder};
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
        .unwrap_or(443);
    ("0.0.0.0", port)
}

fn cert() -> SslAcceptorBuilder {
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("nopass.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();
    builder
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
        .bind_openssl((addr, port), cert())?
        .run()
        .await
}