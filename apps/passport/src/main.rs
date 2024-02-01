use tonic::transport::Server;
use makoto_grpc::pkg::passport::passport_rpc_server::PassportRpcServer;
use makoto_logger::{info, Logger};
use makoto_lib::Result as AnyResult;

mod service;
mod repo;

#[tokio::main]
async fn main() -> AnyResult<()> {

    Logger::init()?;

    let hosts = makoto_config::hosts::Hosts::new();
    let addr = hosts.passport.parse()?;

    let db = makoto_db::new().await.expect("cannot open database connection");
    let credentials_repo = repo::Repo::new(db);

    let passport_service = service::PassportRpcServiceImplementation::new(credentials_repo);
    let passport_service = PassportRpcServer::new(passport_service);

    info!("server start! host: {}", addr);

    Server::builder()
        .add_service(passport_service)
        .serve(addr)
        .await?;

    Ok(())
}