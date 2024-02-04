use tonic::{Request, Response, Status};
use uuid::Uuid;
use makoto_db::repo::credentials::GetCredentialsRecordBy;
use makoto_grpc::pkg::passport as rpc;
use makoto_grpc::pkg::passport::{CreateUserRequest, CreateUserResponse, GetPublicUserRequest, GetPublicUserResponse};
use makoto_lib::errors::{HandleError, SafeUnwrapWithMessage};

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
            password: req.password,
            provider_id: req.provider_id,
        }).await.handle()?;

        Ok(Response::new(CreateUserResponse {
            user_id: user.id.into()
        }))
    }

    async fn get_public_user(&self, req: Request<GetPublicUserRequest>) -> Result<Response<GetPublicUserResponse>, Status> {
        let req = req.into_inner();


        let get_by = match req.get_user_by.safe_unwrap("should specify `get_by`")? {
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