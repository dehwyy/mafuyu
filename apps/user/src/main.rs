use tonic::transport::Server;
use makoto_grpc::pkg::user::user_rpc_server::UserRpcServer;
use makoto_logger::{info, Logger};
use makoto_lib::Result as AnyResult;

mod service;
mod repo;
mod tools;

#[tokio::main]
async fn main() -> AnyResult<()> {

    Logger::init()?;

    let hosts = makoto_config::hosts::Hosts::new();
    let addr = hosts.user.parse()?;

    let db = makoto_db::new().await.expect("cannot open database connection");

    let user_repo = repo::user::UserRepo::new(db.clone());
    let languages_repo  = repo::languages::LanguagesRepo::new(db.clone());

    let user_service = service::UserRpcServiceImplementation::new(user_repo, languages_repo).await;
    let user_service = UserRpcServer::new(user_service);

    info!("server start! host: {}", addr);

    Server::builder()
        .add_service(user_service)
        .serve(addr)
        .await?;

    Ok(())
}
