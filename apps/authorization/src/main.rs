mod service;

use tonic::transport::Server;
use service::AuthorizationRpcServiceImplementation;


use makoto_grpc::pkg::authorization::authorization_rpc_server::AuthorizationRpcServer;
use makoto_logger::{info, Logger};
use makoto_lib::Result as AnyResult;

#[tokio::main]
async fn main() -> AnyResult<()> {

    Logger::init()?;

    let hosts = makoto_config::hosts::Hosts::new();
    let addr = hosts.authorization.parse()?;

    let db = makoto_db::new().await.expect("cannot open database connection");

    let redis = redis::Client::open(makoto_config::constants::redis::AUTH_URL).expect("cannot open redis connection");

    let auth_service= AuthorizationRpcServiceImplementation::new(redis).await;
    let auth_service = AuthorizationRpcServer::new(auth_service);

    info!("server start! host: {}", addr);

    Server::builder()
        .add_service(auth_service)
        .serve(addr)
        .await?;

    Ok(())
}
