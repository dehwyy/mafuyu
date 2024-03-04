mod service;
mod oauth2;


use makoto_grpc::pkg::oauth2::o_auth2_rpc_server::OAuth2RpcServer;
use logger::*;

fn main() {
    let cfg = makoto_config::secrets::Secrets::new();

    let _guard = mafuyu_sentry::Sentry::new(cfg.sentry_dsn);

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            runtime().await.unwrap();
        })
}
async fn runtime() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = makoto_config::secrets::Secrets::new();
    Logger::new(cfg.environment);

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

