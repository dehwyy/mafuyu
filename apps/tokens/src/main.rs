mod service;
mod jwt;
mod repo;

use makoto_grpc::pkg::tokens::tokens_rpc_server::TokensRpcServer;
use logger::*;
use makoto_lib::Result as AnyResult;

#[tokio::main]
async fn main() -> AnyResult<()> {
    let cfg = makoto_config::secrets::Secrets::new();
    Logger::new(cfg.environment);

    let hosts = makoto_config::hosts::Hosts::new();
    let addr = hosts.tokens.parse()?;

    info!("server {}", makoto_config::constants::nats::TOKENS_SERVER);

    let nats = async_nats::connect(makoto_config::constants::nats::TOKENS_SERVER).await?;

    let db = makoto_db::new().await.unwrap();
    let token_repo = repo::Repo::new(db);

    let rpc_clients = makoto_grpc::RpcClients::get_all_client().await;

    let tokens_service = service::TokensRpcServiceImplementation::new(
        service::TokensRpcServiceImplementation{
            token_repo,
            nats_client: nats,
            oauth2_client: rpc_clients.oauth2_client.unwrap(),
            integrations_client: rpc_clients.integrations_client.unwrap()
        }
    );
    let tokens_service = TokensRpcServer::new(tokens_service);

    info!("server started on host: {}!", addr);

    tonic::transport::Server::builder()
        .add_service(tokens_service)
        .serve(addr)
        .await?;

    Ok(())
}

