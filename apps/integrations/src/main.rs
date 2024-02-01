use tonic::transport::Server;
use makoto_grpc::pkg::integrations::integrations_rpc_server::IntegrationsRpcServer;
use makoto_logger::{info, Logger};

mod service;
mod provider;


use makoto_lib::Result as AnyResult;

#[tokio::main]
async fn main() -> AnyResult<()> {

    Logger::init()?;

    let hosts = makoto_config::hosts::Hosts::new();
    let addr = hosts.integrations.parse()?;

    let db = makoto_db::new().await.expect("cannot open database connection");

    let integration_service = service::IntegrationRpcServiceImplementation::new();
    let integration_service = IntegrationsRpcServer::new(integration_service);

    info!("server start! host: {}", addr);

    Server::builder()
        .add_service(integration_service)
        .serve(addr)
        .await?;

    Ok(())
}