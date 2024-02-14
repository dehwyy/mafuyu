use std::borrow::BorrowMut;
use tonic::{Request, Response, Status};
use uuid::Uuid;

use makoto_lib::errors::prelude::*;

use makoto_grpc::errors::GrpcHandleError;
use makoto_grpc::pkg::general::IsOkResponse;
use makoto_grpc::pkg::tokens::{self as rpc, SaveTokensRequest, tokens_rpc_server::TokensRpc};
use makoto_grpc::pkg::oauth2;
use makoto_grpc::pkg::integrations;

use mafuyu_nats::payload::tokens as nats_tokens;
use mafuyu_nats::message::Encoder as NatsJsonEncoder;
use makoto_grpc::pkg::integrations::GetBasicUserRequest;
use makoto_lib::errors::RepositoryError;

use crate::jwt::{Jwt, JwtPayload, TokenError};
use crate::repo::GetTokenRecordBy;

pub struct TokensRpcServiceImplementation<T = tonic::transport::Channel> {
    pub token_repo: crate::repo::Repo,
    pub nats_client: async_nats::Client,
    pub oauth2_client: oauth2::o_auth2_rpc_client::OAuth2RpcClient<T>,
    pub integrations_client: integrations::integrations_rpc_client::IntegrationsRpcClient<T>
}

impl TokensRpcServiceImplementation {
    pub fn new(slf: Self) -> Self {
        Self {
            ..slf
        }
    }

    fn try_as_uuid(&self, uuid: &str) -> Result<Uuid, Status> {
        Uuid::try_parse(&uuid).invalid_argument_error()
    }
}

#[tonic::async_trait]
impl TokensRpc for TokensRpcServiceImplementation {
    async fn generate_token_pair(&self, req: Request<rpc::GenerateTokenPairRequest>) -> Result<Response<rpc::GenerateTokenPairResponse>, Status> {
        let req = req.into_inner();
        let user_id = self.try_as_uuid(&req.user_id)?;

        let record = self.token_repo.get_token_record(GetTokenRecordBy::UserId(user_id)).await;

        // Clarify whether record either `exists` or `not_found`.
        // If common error occurred => return RpcError (Status)
        if let Some(has_error) = record.as_ref().map_err(|err| match err {
            RepositoryError::NotFound(_) => Ok(()),
                err => Err(err)
        }).err() {
            has_error.map_err(|err| err.clone()).handle()?
        }


        let jwt_payload = JwtPayload {
            user_id: req.user_id.clone()
        };

        let access_token = Jwt::new_access_token(jwt_payload.clone()).internal_error()?;

        // `is_error` is actually `is_not_found` as common errors were handled above (right after `get_token_record`)
        let refresh_token = match record.is_err() {
            true => Jwt::new_refresh_token(jwt_payload),
            // Both `record` and `refresh_token` should be defined as record is !!found => found, and `provider` is None (not `oauth2`)
            // Should use `unwrap` instead of `handle`, but I guess it's fine.
            false => match record.handle()?.refresh_token {
                Some(token) => Ok(token),
                None => Err(TokenError::Internal)
            }
        }.internal_error()?;


        self.token_repo.insert_tokens(user_id, access_token.clone(), Some(refresh_token.clone()), None).await.handle()?;

        Ok(Response::new(rpc::GenerateTokenPairResponse {
            access_token,
            refresh_token
        }))
    }

    async fn validate_token(&self, req: Request<rpc::ValidateTokenRequest>) -> Result<Response<rpc::ValidateTokenResponse>, Status> {
        let req = req.into_inner();

        let access_token = req.access_token.clone();

        let provider_name = match req.provider {
            Some(provider_name) => Some(provider_name),
            None => {
                self.token_repo.get_token_record(GetTokenRecordBy::AccessToken(access_token)).await.handle()?.provider
            }
        };

        let user_id = match provider_name {
            Some(oauth2_provider) => {
                self.integrations_client.clone().borrow_mut().get_basic_user(Request::new(GetBasicUserRequest {
                    provider: oauth2_provider,
                    access_token: req.access_token
                })).await.map(|_| Option::<String>::None)?
                // `oauth2_tokens` doesn't have `mafuyu user_id` in their claims,
                // (for mafuyu tokens `Jwt service` gets `user_id` from claims).
            },
            None => Jwt::validate_access_token(req.access_token)
                .map(|v| Some(v.user_id)).unauthenticated_error()?
        };

        Ok(Response::new(rpc::ValidateTokenResponse {
            user_id
        }))
    }

    async fn clear_tokens(&self, req: Request<rpc::ClearTokensRequest>) -> Result<Response<IsOkResponse>, Status> {
        let req = req.into_inner();

        let payload = NatsJsonEncoder::encode(nats_tokens::ClearTokensRequest {
            access_token: req.access_token,
            user_id: req.user_id
        }).handle()?;

        self.nats_client.publish(nats_tokens::subject::CLEAR_TOKENS, payload.into()).await.handle()?;


        Ok(Response::new(IsOkResponse {
            is_ok: true
        }))
    }

    async fn save_tokens(&self, req: Request<SaveTokensRequest>) -> Result<Response<IsOkResponse>, Status> {
        let req = req.into_inner();

        let user_id = Uuid::try_parse(&req.user_id).invalid_argument_error()?;

        self.token_repo.insert_tokens(user_id, req.access_token, req.refresh_token, req.provider).await.handle()?;

        Ok(Response::new(IsOkResponse {
            is_ok: true
        }))
    }

    async fn refresh_the_token(&self, req: Request<rpc::RefreshTheTokenRequest>) -> Result<Response<rpc::RefreshTheTokenResponse>, Status> {
        let req = req.into_inner();

        let record = self.token_repo.get_token_record(GetTokenRecordBy::RefreshToken(req.refresh_token.clone())).await.handle()?;

        let access_token = match record.provider {
            Some(oauth2_provider) => {
                let response = self.oauth2_client.clone().borrow_mut().refresh_the_token(oauth2::RefreshTheOAuth2TokenRequest {
                    refresh_token: req.refresh_token.clone(),
                    provider: oauth2_provider
                }).await?.into_inner();

                Ok(response.access_token)
            },
            None => {
                Jwt::new_access_token(JwtPayload {
                    user_id: record.user_id.to_string()
                }).internal_error()
            }
        }?;

        Ok(Response::new(rpc::RefreshTheTokenResponse {
            refresh_token: req.refresh_token,
            access_token
        }))
    }
}