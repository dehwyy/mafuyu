use makoto_grpc::errors::GrpcHandleError;
use tonic::{Request, Response, Status};
use uuid::Uuid;
use makoto_db::repo::credentials::GetCredentialsRecordBy;
use makoto_grpc::pkg::passport as rpc;
use makoto_grpc::pkg::passport::{CreateUserPassportResponse, CreateUserPassportRequest, GetPublicUserRequest, GetPublicUserResponse, UpdateUsernameRequest};
use mafuyu_lib::errors::prelude::*;

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
    async fn create_user(&self, req: Request<CreateUserPassportRequest>) -> Result<Response<CreateUserPassportResponse>, Status> {
        let req = req.into_inner();

        let user = self.credentials_repo.create_user(credentials_repo::CreateUserPayload {
            username: req.username,
            email: req.email,
            password: req.password,
            provider_id: req.provider_id,
        }).await.handle()?;

        Ok(Response::new(CreateUserPassportResponse {
            user_id: user.id.into()
        }))
    }

    async fn update_username(&self, req: Request<UpdateUsernameRequest>) -> Result<Response<()>, tonic::Status> {
        let req =  req.into_inner();

        let user_id = Uuid::try_parse(&req.user_id).invalid_argument_error()?;

        self.credentials_repo.update_username(user_id, req.username).await.handle()?;

        Ok(Response::new(()))
    }

    async fn get_public_user(&self, req: Request<GetPublicUserRequest>) -> Result<Response<GetPublicUserResponse>, Status> {
        let req = req.into_inner();


        let get_by = match req.get_user_by.unwrap_or_not_found("should specify `get_by`")? {
            rpc::get_public_user_request::GetUserBy::ProviderId(id) => GetCredentialsRecordBy::ProviderId(id),
            rpc::get_public_user_request::GetUserBy::Username(id) => GetCredentialsRecordBy::Username(id),
            rpc::get_public_user_request::GetUserBy::UserId(id) => GetCredentialsRecordBy::UserId(
                Uuid::try_parse(&id).map_err(|err| Status::invalid_argument(err.to_string()))?
            )
        };

        let user = self.credentials_repo.get_user(get_by).await.handle()?;

        Ok(Response::new(GetPublicUserResponse {
            user_id: user.id.to_string(),
            provider_id: user.provider_id,
            username: user.username,
        }))
    }
}
