mod service;
mod interceptors;
mod middleware;

use interceptors::remove_keep_alive::RemoveKeepAliveHeaderInterceptor;

use std::str::FromStr;
use std::time::Duration;

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
    let api_service = ApiRpcServer::with_interceptor(api, RemoveKeepAliveHeaderInterceptor::intercept);

    info!("server start! host: {}", addr);

    let exposed_headers = vec!("grpc-status", "grpc-message", "grpc-status-details-bin", "x-access-token", "x-refresh-token", "set-cookie")
        .iter()
        .map(|s| HeaderName::from_str(s).expect("invalid header name"))
        .collect::<Vec<HeaderName>>();

    // todo
    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::any())
        .max_age(std::time::Duration::from_secs(1))
        .expose_headers(exposed_headers)
        .allow_headers(AllowHeaders::any());

    let set_tokens_cookies = middleware::set_tokens_cookies::SetTokensCookies::new();

    let app_layer = tower::ServiceBuilder::new()
        .timeout(Duration::from_secs(15))
        .layer(middleware::BaseMiddlewareLayer::new(set_tokens_cookies))
        .layer(tonic::service::interceptor(RemoveKeepAliveHeaderInterceptor::intercept))
        .layer(cors)
        .layer(GrpcWebLayer::new())
        .into_inner();

    Server::builder()
        .accept_http1(true)
        .layer(app_layer)
        .add_service(api_service)
        .serve(addr)
        .await?;


    Ok(())
}
