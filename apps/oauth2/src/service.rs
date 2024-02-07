use crate::oauth2::{OAuth2, OAuth2Provider, OAuth2ProviderName, RefreshError};

use tonic::{Request, Response, Status};

use makoto_grpc::pkg::oauth2 as rpc;
use makoto_grpc::pkg::oauth2::o_auth2_rpc_server::OAuth2Rpc;
use makoto_grpc::errors::GrpcHandleError;

pub struct OAuth2RpcServiceImplementation {
    pub oauth2: OAuth2
}

impl OAuth2RpcServiceImplementation {
    pub fn new(v: Self) -> Self {
        Self {
            ..v
        }
    }
    fn get_provider(&self, provider: &str) -> Result<&impl OAuth2Provider, Status> {
        let provider_name  = match provider {
            "github" => Ok(OAuth2ProviderName::Github),
            "google" => Ok(OAuth2ProviderName::Google),
            _ => Err("cannot infer provider".to_string())
        }.invalid_argument_error()?;

        Ok(self.oauth2.get_provider(provider_name))
    }
}

#[tonic::async_trait]
impl OAuth2Rpc for OAuth2RpcServiceImplementation {
    async fn create_redirect_url(&self, request: Request<rpc::CreateRedirectUrlRequest>) -> Result<Response<rpc::CreateRedirectUrlResponse>, Status> {
        let req = request.into_inner();

        let provider = self.get_provider(&req.provider)?;

        let (redirect_url, csrf_token) = provider.create_redirect_url();

        Ok(Response::new(
            rpc::CreateRedirectUrlResponse {
                redirect_url,
                csrf_token: csrf_token.secret().to_string()
            }
        ))
    }

    async fn exchange_code_to_token(&self, request: Request<rpc::ExchangeCodeToTokenRequest>) -> Result<Response<rpc::ExchangeCodeToTokenResponse>, Status> {
        let req = request.into_inner();

        let provider = self.get_provider(&req.provider)?;

        let oauth2_token = provider.exchange_code_to_token(req.code).await.unauthenticated_error()?;

        Ok(Response::new(
            rpc::ExchangeCodeToTokenResponse {
                access_token: oauth2_token.access_token,
                refresh_token: oauth2_token.refresh_token
            }
        ))

    }

    async fn refresh_the_token(&self, request: Request<rpc::RefreshTheTokenRequest>) -> Result<Response<rpc::RefreshTheTokenResponse>, Status> {
        let req = request.into_inner();

        let provider = self.get_provider(&req.provider)?;

        let oauth2_token = match  provider.refresh().await {
            Ok(token) => Ok(token),
            Err(refresh_token) => Err(match refresh_token {
                    RefreshError::Internal => Status::internal("cannot refresh the token"),
                    RefreshError::NotSupported => Status::unimplemented("oauth2 provider doesn't support token refresh")
                })
        }?;

        // TODO: maybe returns both refresh and access tokens
        Ok(Response::new(
            rpc::RefreshTheTokenResponse {
                access_token: oauth2_token.access_token
            }
        ))
    }
}
