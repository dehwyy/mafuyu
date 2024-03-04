use tonic::transport::Server;
use makoto_grpc::pkg::user::user_rpc_server::UserRpcServer;
use logger::{info, Logger};

mod service;
mod repo;
mod tools;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = makoto_config::secrets::Secrets::new();
    Logger::new(cfg.environment);

    let hosts = makoto_config::hosts::Hosts::new();
    let addr = hosts.user.parse()?;

    let db = makoto_db::new().await.expect("cannot open database connection");

    let user_repo = repo::user::UserRepo::new(db.clone());
    let languages_repo  = repo::languages::LanguagesRepo::new(db.clone());
    let followers_friends_repo = repo::followers_friends::FollowersFriendsRepo::new(db.clone());
    let blocked_user_repo = repo::blocked_users::BlockedUsersRepo::new(db.clone());

    let user_service = service::UserRpcServiceImplementation::new(user_repo, languages_repo, followers_friends_repo, blocked_user_repo).await;
    let user_service = UserRpcServer::new(user_service);

    info!("server start! host: {}", addr);

    Server::builder()
        .add_service(user_service)
        .serve(addr)
        .await?;

    Ok(())
}
