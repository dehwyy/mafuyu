mod repository;
mod utils;
mod service;

use tonic::transport::Server;
use service::AuthRpcServiceImplementation;


use makoto_grpc::pkg::auth::auth_rpc_server::AuthRpcServer;
use logger::{info, Logger};
use makoto_lib::Result as AnyResult;

#[tokio::main]
async fn main() -> AnyResult<()> {

    Logger::new();

    let hosts = makoto_config::hosts::Hosts::new();
    let addr = hosts.auth.parse()?;

    let db = makoto_db::new().await.expect("cannot open database connection");
    
    let credentials_repo = repository::credentials::Credentials::new(db.clone());
    let token_repo  =repository::tokens::Tokens::new(db.clone());

    let redis = redis::Client::open(makoto_config::constants::redis::AUTH_URL).expect("cannot open redis connection");
    
    let auth_service= AuthRpcServiceImplementation::new(credentials_repo, token_repo, redis).await;
    let auth_service = AuthRpcServer::new(auth_service);

    info!("server start! host: {}", addr);

    Server::builder()
        .add_service(auth_service)
        .serve(addr)
        .await?;

    Ok(())
}
