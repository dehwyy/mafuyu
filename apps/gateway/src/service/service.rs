use std::borrow::BorrowMut;

use makoto_grpc::{pkg as grpc, Tools};
use grpc::{api::api_rpc_server, auth::auth_rpc_client, oauth2::o_auth2_rpc_client, tokens::tokens_rpc_client};
use grpc::auth::{AuthenticationResponse, SignInWithTokenRequest};
use grpc::oauth2::{CreateRedirectUrlRequest, CreateRedirectUrlResponse, ExchangeCodeToTokenRequest, ExchangeCodeToTokenResponse} ;
use grpc::tokens::RefreshTheTokenRequest;


use makoto_grpc::pkg::general::IsOkResponse;

use makoto_logger::{info, warn};
use tonic::{Request, Response, Status};
pub struct ApiRpcServiceImplementation<T = tonic::transport::Channel> {
  auth_client: auth_rpc_client::AuthRpcClient<T>,
  tokens_client: tokens_rpc_client::TokensRpcClient<T>,
  oauth2_client: o_auth2_rpc_client::OAuth2RpcClient<T>
}

impl ApiRpcServiceImplementation {

  pub async fn new() -> Self {

    let clients = makoto_grpc::RpcClients::get_all_client().await;

    Self {
      auth_client: clients.auth_client.unwrap(),
      tokens_client: clients.tokens_client.unwrap(),
      oauth2_client: clients.oauth2_client.unwrap()
    }
  }
}

#[tonic::async_trait]
impl api_rpc_server::ApiRpc for ApiRpcServiceImplementation {
  async fn sign_up(&self, req: Request<grpc::auth::SignUpRequest>) -> Result<Response<AuthenticationResponse>, Status> {

    // cloning client @see (https://docs.rs/tonic/latest/tonic/client/index.html#concurrent-usage)
    let service_response = self.auth_client.clone().borrow_mut().sign_up(req).await?.into_inner();

    let response = Tools::attach_tokens(
      service_response.access_token, Some(service_response.refresh_token),
      AuthenticationResponse {
        username: service_response.username,
        user_id: service_response.user_id
    });

    Ok(response)
  }

  async fn sign_in(&self, req: Request<grpc::auth::SignInRequest>) -> Result<Response<AuthenticationResponse>, Status> {
    let r = self.auth_client.clone().borrow_mut().sign_in(req).await?.into_inner();

    let response = Tools::attach_tokens(
      r.access_token, Some(r.refresh_token),
      AuthenticationResponse {
        username: r.username,
        user_id: r.user_id
      }
    );

    Ok(response)
  }

  async fn sign_in_with_token(&self, req: Request<SignInWithTokenRequest>) -> Result<Response<AuthenticationResponse>, Status> {

    let (metadata, _, req) = req.into_parts();

    // default `token` value is from request
    let mut token = req.token;

    // But, if `x-access-token` header is present, it will be used instead
    if  let Some(Ok(t)) = metadata.get("x-access-token").map(|t| t.to_str()) {
       token = t.to_string();
    }

    let response = self.auth_client.clone().borrow_mut().sign_in_with_token(
      Request::new( SignInWithTokenRequest { token })
    ).await?.into_inner();

    let response = Tools::attach_tokens(
      response.access_token, Some(response.refresh_token),
      AuthenticationResponse {
        username: response.username,
        user_id: response.user_id
      }
    );

    Ok(response)
  }

  async fn sign_out(&self, req: Request<grpc::auth::SignOutRequest>) -> Result<Response<IsOkResponse>, Status> {
    self.auth_client.clone().borrow_mut().sign_out(req).await
  }

  async fn refresh_the_token(&self, req: Request<RefreshTheTokenRequest>) -> Result<Response<IsOkResponse>, Status> {
    let response = self.tokens_client.clone().borrow_mut().refresh_the_token(req).await?.into_inner();

    let response = Tools::attach_tokens(
      response.access_token, Some(response.refresh_token),
      IsOkResponse {is_ok: true }
    );

    Ok(response)
  }

  async fn create_o_auth2_redirect_url(&self, req: Request<CreateRedirectUrlRequest>) -> Result<Response<CreateRedirectUrlResponse>, Status> {
    self.oauth2_client.clone().borrow_mut().create_redirect_url(req).await
  }

  async fn exchange_o_auth2_code_to_token(&self, request: Request<ExchangeCodeToTokenRequest>) -> Result<Response<ExchangeCodeToTokenResponse>, Status> {
    self.oauth2_client.clone().borrow_mut().exchange_code_to_token(request).await
  }
}
