use std::borrow::BorrowMut;

use makoto_grpc::pkg::api_auth::*;
use makoto_grpc::pkg::api::api_rpc_server::ApiRpc;
use makoto_grpc::pkg::general::BoolStatus;
use makoto_grpc::Result as GrpcResult;

use makoto_lib::errors::prelude::*;

use makoto_logger::{info, warn};

use tonic::{Request as Req, Response};

pub struct ApiRpcServiceImplementation {
  auth_client: makoto_grpc::pkg::api_auth::auth_rpc_client::AuthRpcClient<tonic::transport::Channel>,
}

impl ApiRpcServiceImplementation {

  pub async fn new() -> Self {
    let hosts = makoto_config::hosts::Hosts::new();

    let with_http = |url: &str| format!("http://{}", url);

    let auth_client = auth_rpc_client::AuthRpcClient::connect(with_http(&hosts.auth)).await.unwrap();

    Self {
      auth_client
    }
  }
}

#[tonic::async_trait]
impl ApiRpc for ApiRpcServiceImplementation {
  async fn sign_up(&self, req: Req<SignUpRequest>) -> GrpcResult<AuthenticationResponse> {

    // cloning client @see (https://docs.rs/tonic/latest/tonic/client/index.html#concurrent-usage)
    let service_response = self.auth_client.clone().borrow_mut().sign_up(req).await?;

    // under the hood fn, that implements `IntoTrait` attaches `Access, Refresh` tokens to Metadata, other fields are stored in `Response`
    Ok(service_response.into_inner().into())

  }

  async fn sign_in(&self, req: Req<SignInRequest>) -> GrpcResult<AuthenticationResponse> {
    let service_response = self.auth_client.clone().borrow_mut().sign_in(req).await?;

    Ok(service_response.into_inner().into())
  }

  async fn refresh(&self, req: Req<RefreshRequest>) -> GrpcResult<AuthenticationResponse> {
    let service_response = self.auth_client.clone().borrow_mut().refresh(req).await?;

    Ok(service_response.into_inner().into())
  }

  async fn create_o_auth2_redirect_url(&self, req: Req<OAuth2RedirectUrlRequest>) -> GrpcResult<OAuth2RedirectUrlResponse> {
    self.auth_client.clone().borrow_mut().create_o_auth2_redirect_url(req).await
  }

  async fn sign_in_oauth(&self, req: Req<SignInOauthRequest>) -> GrpcResult<AuthenticationResponse> {
    let service_response = self.auth_client.clone().borrow_mut().sign_in_oauth(req).await?;

    Ok(service_response.into_inner().into())
  }

  async fn sign_in_token(&self, req: Req<SignInTokenRequest>) -> GrpcResult<AuthenticationResponse> {

    let (metadata, _, req) = req.into_parts();

    // default `token` value is from request
    let mut token = req.token;

    // But, if `x-access-token` header is present, it will be used instead
    if  let Some(Ok(t)) = metadata.get("x-access-token").map(|t| t.to_str()) {
       token = t.to_string();
    }

    let service_response = self.auth_client.clone().borrow_mut().sign_in_token(
      Req::new( SignInTokenRequest { token })
    ).await?;

    Ok(service_response.into_inner().into())
  }

  async fn sign_out(&self, req: Req<SignOutRequest>) -> GrpcResult<BoolStatus> {
    self.auth_client.clone().borrow_mut().sign_out(req).await
  }

  async fn confirm_mail_by_code(&self, req: Req<ConfirmMailByCodeRequest>) -> GrpcResult<AuthenticationResponse> {
    todo!()
  }

  async fn proceed_to_update_password(&self, req: Req<ProceedToUpdatePasswordRequest>) -> GrpcResult<ProceedToUpdatePasswordResponse> {
    todo!()
  }

  async fn proceed_to_recover_password(&self, req: Req<ProceedToRecoverPasswordRequest>) -> GrpcResult<BoolStatus> {
    todo!()
  }

  async fn submit_new_password_by_code(&self, req: Req<SubmitNewPasswordByCodeRequest>) -> GrpcResult<AuthenticationResponse> {
    todo!()
  }

  async fn is_email_available(&self, req: Req<IsEmailAvailableRequest>) -> GrpcResult<BoolStatus> {
    self.auth_client.clone().borrow_mut().is_email_available(req).await
  }

  async fn is_username_available(&self, req: Req<IsUsernameAvailableRequest>) -> GrpcResult<BoolStatus> {
    self.auth_client.clone().borrow_mut().is_username_available(req).await
  }
}
