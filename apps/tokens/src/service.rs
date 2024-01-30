use std::borrow::BorrowMut;
use tonic::{Request, Response, Status};
use uuid::Uuid;

use makoto_lib::errors::prelude::*;

use makoto_grpc::pkg::general::IsOkResponse;
use makoto_grpc::pkg::tokens as rpc;
use makoto_grpc::pkg::tokens::RefreshTheTokenRequest;
use makoto_grpc::pkg::tokens::tokens_rpc_server::TokensRpc;
use makoto_grpc::pkg::oauth2;
use makoto_grpc::pkg::oauth2::OAuth2Provider;

use crate::jwt::{Jwt, JwtPayload, TokenError};
use crate::repo::GetTokenRecordBy;

pub struct TokensRpcServiceImplementation<T = tonic::transport::Channel> {
    pub token_repo: crate::repo::Repo,
    pub oauth2_client: oauth2::o_auth2_rpc_client::OAuth2RpcClient<T>
}

impl TokensRpcServiceImplementation {
    pub fn new(slf: Self) -> Self {
        Self {
            ..slf
        }
    }

    fn try_as_uuid(&self, uuid: &str) -> Result<Uuid, Status> {
        Uuid::try_parse(&uuid).map_err(|err| {
            format!("[cannot parse user_id as uuid]: {}", err.to_string())
        }).invalid_argument_error()
    }
}

#[tonic::async_trait]
impl TokensRpc for TokensRpcServiceImplementation {
    async fn generate_token_pair(&self, req: Request<rpc::GenerateTokenPairRequest>) -> Result<Response<rpc::GenerateTokenPairResponse>, Status> {
        let req = req.into_inner();
        let user_id = self.try_as_uuid(&req.user_id)?;

        let record = self.token_repo.get_token_record(GetTokenRecordBy::UserId(user_id)).await;

        let jwt_payload = JwtPayload {
            user_id: req.user_id.clone()
        };

        let access_token = Jwt::new_access_token(jwt_payload.clone()).map_err(|err| err.to_string()).internal_error()?;

        let refresh_token = match record.is_not_found() {
            true => Jwt::new_refresh_token(jwt_payload),
            // Both `record` and `refresh_token` should be defined as as record is !!found => found, and \
            // `native `token provider
            false => match record.unwrap().refresh_token {
                Some(token) => Ok(token),
                None => Err(TokenError::Internal)
            }
        }.map_err(|err| err.to_string() ).internal_error()?;


        self.token_repo.insert_tokens(user_id, &access_token, &refresh_token).await.handle()?;

        Ok(Response::new(rpc::GenerateTokenPairResponse {
            access_token,
            refresh_token
        }))
    }

    async fn validate_token(&self, req: Request<rpc::ValidateTokenRequest>) -> Result<Response<rpc::ValidateTokenResponse>, Status> {
        let req = req.into_inner();

        let record = self.token_repo.get_token_record(GetTokenRecordBy::AccessToken(req.access_token.clone())).await.handle()?;

        let is_valid = match record.provider {
            Some(oauth2_provider) => todo!(), // request,
            None => Jwt::validate_access_token(req.access_token).is_ok()
        };

        Ok(Response::new(rpc::ValidateTokenResponse {
            is_token_valid: is_valid
        }))
    }

    async fn clear_tokens(&self, request: Request<rpc::ClearTokensRequest>) -> Result<Response<IsOkResponse>, Status> {
        todo!() // NATS REQUEST
    }

    async fn refresh_the_token(&self, req: Request<rpc::RefreshTheTokenRequest>) -> Result<Response<rpc::RefreshTheTokenResponse>, Status> {
        let req = req.into_inner();

        let record = self.token_repo.get_token_record(GetTokenRecordBy::RefreshToken(req.refresh_token.clone())).await.handle()?;

        let access_token = match record.provider {
            Some(oauth2_provider) => {
                let provider = OAuth2Provider::from_str_name(&oauth2_provider).safe_unwrap("invalid oauth2 provider")?;

                let response = self.oauth2_client.clone().borrow_mut().refresh_the_token(oauth2::RefreshTheTokenRequest {
                    refresh_token: req.refresh_token.clone(),
                    provider: provider as i32
                }).await?.into_inner();

                Ok(response.access_token)
            },
            None => {
                Jwt::new_access_token(JwtPayload {
                    user_id: record.user_id.to_string()
                }).map_err(|err| err.to_string()).internal_error()
            }
        }?;

        Ok(Response::new(rpc::RefreshTheTokenResponse {
            refresh_token: req.refresh_token,
            access_token
        }))
    }
}