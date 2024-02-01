use tonic::{Request, Response, Status};
use makoto_grpc::pkg::passport as rpc;
use makoto_grpc::pkg::passport::{CreateUserRequest, CreateUserResponse};
use makoto_lib::errors::HandleError;

use crate::repo as credentials_repo;

pub struct PassportRpcServiceImplementation {
    credentials_repo: crate::repo::Repo,
}

impl PassportRpcServiceImplementation {
    pub fn new(credentials_repo: crate::repo::Repo) -> Self {
        Self {
            credentials_repo
        }
    }
}

#[tonic::async_trait]
impl rpc::passport_rpc_server::PassportRpc for PassportRpcServiceImplementation {
    async fn create_user(&self, req: Request<CreateUserRequest>) -> Result<Response<CreateUserResponse>, Status> {
        let req = req.into_inner();

        let user = self.credentials_repo.create_user(credentials_repo::CreateUserPayload {
            username: req.username,
            email: req.email,
            password: req.password
        }).await.handle()?;

        Ok(Response::new(CreateUserResponse {
            user_id: user.id.into()
        }))
    }
}