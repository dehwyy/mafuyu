mod service;
mod oauth2;


use makoto_grpc::pkg::oauth2::o_auth2_rpc_server::OAuth2RpcServer;
use logger::*;
use makoto_lib::Result as AnyResult;

#[tokio::main]
async fn main() -> AnyResult<()> {
    Logger::new();

    let hosts = makoto_config::hosts::Hosts::new();
    let addr = hosts.oauth2.parse()?;

    let oauth2_service = service::OAuth2RpcServiceImplementation::new(
        service::OAuth2RpcServiceImplementation{
            oauth2: oauth2::OAuth2::new()
        }
    );
    let oauth2_service = OAuth2RpcServer::new(oauth2_service);

    info!("server started on host: {}!", addr);

    tonic::transport::Server::builder()
        .add_service(oauth2_service)
        .serve(addr)
        .await?;

    Ok(())
}

