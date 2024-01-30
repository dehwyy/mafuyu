mod service;
mod jwt;
mod repo;

use makoto_grpc::pkg::tokens::tokens_rpc_server::TokensRpcServer;
use makoto_logger::*;
use makoto_lib::Result as AnyResult;

#[tokio::main]
async fn main() -> AnyResult<()> {
    Logger::init()?;

    let hosts = makoto_config::hosts::Hosts::new();
    let addr = hosts.tokens.parse()?;

    let db = makoto_db::new().await.unwrap();
    let token_repo = repo::Repo::new(db);

    let rpc_clients = makoto_grpc::RpcClients::get_all_client().await;

    let tokens_service = service::TokensRpcServiceImplementation::new(
        service::TokensRpcServiceImplementation{
            token_repo,
            oauth2_client: rpc_clients.oauth2_client.unwrap()
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

