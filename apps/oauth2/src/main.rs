mod handler;
mod oauth2;

use actix_web::{App, HttpServer, Responder};
use makoto_logger::{info, Logger};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    Logger::init().unwrap();
    let config = makoto_config::hosts::Hosts::new();

    info!("server started: {}", config.oauth2);

    HttpServer::new(|| {
        App::new().service(handler::Service::new())
    })
        .bind(config.oauth2)?
        .run()
        .await
}
