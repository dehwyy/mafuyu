mod repo;
mod service;
mod db;

use makoto_grpc::pkg::cdn::cdn_rpc_server::CdnRpcServer;
use logger::{info, Logger};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = makoto_config::secrets::Secrets::new();
    Logger::new(cfg.environment);

    let hosts = makoto_config::hosts::Hosts::new();
    let addr = hosts.cdn_rpc.parse()?;

    let db = db::Database::new();
    let repo = repo::Repo::new(db);
    let nats_client = async_nats::connect(makoto_config::constants::nats::CDN_SERVER).await?;

    let cdn_service = service::CdnRpcServiceImplementation::new(nats_client, repo);
    let cdn_service = CdnRpcServer::new(cdn_service);

    info!("server started on host: {}!", addr);

    tonic::transport::Server::builder()
        .add_service(cdn_service)
        .serve(addr)
        .await?;

    Ok(())
}