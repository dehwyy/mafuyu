mod handler;
mod oauth2;

use actix_web::{App, HttpServer};
use actix_cors::Cors;
use makoto_logger::{info, Logger};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    Logger::init().unwrap();
    let config = makoto_config::hosts::Hosts::new();

    info!("server started: {}", config.oauth2);

    HttpServer::new(|| {
        let cors = Cors::default()
            .send_wildcard()
            .allowed_methods(vec!["GET", "POST", "PUT"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT, http::header::CONTENT_TYPE])
            .max_age(360);

        App::new().wrap(cors).service(handler::Service::new())
    })
        .bind(config.oauth2)?
        .run()
        .await
}
