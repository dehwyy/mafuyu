mod service;

use std::str::FromStr;

use tonic::transport::Server;
use service::service::ApiRpcServiceImplementation;

use makoto_grpc::pkg::api::api_rpc_server::ApiRpcServer;
use makoto_logger::{Logger, info};
use tonic_web::{GrpcWebLayer};
use tower_http::cors::{CorsLayer, AllowOrigin, AllowHeaders};
use http::header::HeaderName;


#[tokio::main]
async fn main() -> makoto_lib::Result<()> {

    Logger::init()?;

    let hosts = makoto_config::hosts::Hosts::new();
    let addr = hosts.gateway.parse()?;

    let api = ApiRpcServiceImplementation::new().await;
    let api_service = ApiRpcServer::new(api);

    info!("server start! host: {}", addr);

    let exposed_headers = vec!("grpc-status", "grpc-message", "grpc-status-details-bin", "x-access-token", "x-refresh-token")
        .iter()
        .map(|s| HeaderName::from_str(s).expect("invalid header name"))
        .collect::<Vec<HeaderName>>();

    // todo
    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::any())
        .max_age(std::time::Duration::from_secs(1))
        .expose_headers(exposed_headers)
        .allow_headers(AllowHeaders::any());

    Server::builder()
        .accept_http1(true)
        .layer(cors)
        .layer(GrpcWebLayer::new())
        .add_service(api_service)
        .serve(addr)
        .await?;


    Ok(())
}
