use std::borrow::BorrowMut;
use tonic::{Request, Response, Status};

use makoto_grpc::{pkg as grpc, Tools};
use grpc::api::api_rpc_server;
use grpc::{auth, tokens, oauth2, passport, integrations, user, general, authorization};
use logger::{info, trace};


pub struct ApiRpcServiceImplementation<T = tonic::transport::Channel> {
  auth_client: auth::auth_rpc_client::AuthRpcClient<T>,
  authorization_client: authorization::authorization_rpc_client::AuthorizationRpcClient<T>,
  tokens_client: tokens::tokens_rpc_client::TokensRpcClient<T>,
  oauth2_client: oauth2::o_auth2_rpc_client::OAuth2RpcClient<T>,
  passport_client: passport::passport_rpc_client::PassportRpcClient<T>,
  integrations_client: integrations::integrations_rpc_client::IntegrationsRpcClient<T>,
  user_client: user::user_rpc_client::UserRpcClient<T>,
}

impl ApiRpcServiceImplementation {

  pub async fn new() -> Self {

    let clients = makoto_grpc::RpcClients::get_all_client().await;

    Self {
      auth_client: clients.auth_client.unwrap(),
      authorization_client: clients.authorization_client.unwrap(),
      tokens_client: clients.tokens_client.unwrap(),
      oauth2_client: clients.oauth2_client.unwrap(),
      passport_client: clients.passport_client.unwrap(),
      integrations_client: clients.integrations_client.unwrap(),
      user_client: clients.user_client.unwrap(),
    }
  }
}

#[tonic::async_trait]
impl api_rpc_server::ApiRpc for ApiRpcServiceImplementation {
  async fn sign_up(&self, req: Request<auth::SignUpRequest>) -> Result<Response<auth::AuthenticationResponse>, Status> {

    // cloning client @see (https://docs.rs/tonic/latest/tonic/client/index.html#concurrent-usage)
    let service_response = self.auth_client.clone().borrow_mut().sign_up(req).await?.into_inner();

    let response = Tools::attach_tokens(
      service_response.access_token, Some(service_response.refresh_token),
      auth::AuthenticationResponse {
        username: service_response.username,
        user_id: service_response.user_id
    });

    Ok(response)
  }

  async fn sign_in(&self, req: Request<auth::SignInRequest>) -> Result<Response<auth::AuthenticationResponse>, Status> {
    let r = self.auth_client.clone().borrow_mut().sign_in(req).await?.into_inner();

    let response = Tools::attach_tokens(
      r.access_token, Some(r.refresh_token),
      auth::AuthenticationResponse {
        username: r.username,
        user_id: r.user_id
      }
    );

    Ok(response)
  }

  async fn sign_in_with_token(&self, req: Request<auth::SignInWithTokenRequest>) -> Result<Response<auth::AuthenticationResponse>, Status> {

    let (metadata, _, req) = req.into_parts();

    // default `token` value is from request
    let mut token = req.token;

    // But, if `x-access-token` header is present, it will be used instead
    if  let Some(Ok(t)) = metadata.get(makoto_grpc::METADATA_ACCESS_TOKEN_KEY).map(|t| t.to_str()) {
       token = t.to_string();
    }

    let response = self.auth_client.clone().borrow_mut().sign_in_with_token(
      Request::new( auth::SignInWithTokenRequest { token })
    ).await?.into_inner();

    let response = Tools::attach_tokens(
      response.access_token, Some(response.refresh_token),
      auth::AuthenticationResponse {
        username: response.username,
        user_id: response.user_id
      }
    );

    Ok(response)
  }

  async fn sign_out(&self, req: Request<auth::SignOutRequest>) -> Result<Response<general::IsOkResponse>, Status> {
    self.auth_client.clone().borrow_mut().sign_out(req).await
  }

  async fn send_email_verification_code(&self, req: Request<auth::SendEmailVerificationCodeRequest>) -> Result<Response<()>, Status> {
    self.auth_client.clone().borrow_mut().send_email_verification_code(req).await
  }

  async fn verify_email_code(&self, req: Request<auth::VerifyEmailCodeRequest>) -> Result<Response<general::IsOkResponse>, Status> {
    self.auth_client.clone().borrow_mut().verify_email_code(req).await
  }

  async fn update_username(&self, req: Request<passport::UpdateUsernameRequest>) -> Result<Response<()>, Status> {
    self.passport_client.clone().borrow_mut().update_username(req).await
  }

  async fn refresh_the_token(&self, req: Request<tokens::RefreshTheTokenRequest>) -> Result<Response<general::IsOkResponse>, Status> {
    let response = self.tokens_client.clone().borrow_mut().refresh_the_token(req).await?.into_inner();

    let response = Tools::attach_tokens(
      response.access_token, Some(response.refresh_token),
      general::IsOkResponse {is_ok: true }
    );

    Ok(response)
  }

  async fn create_o_auth2_redirect_url(&self, req: Request<oauth2::CreateRedirectUrlRequest>) -> Result<Response<oauth2::CreateRedirectUrlResponse>, Status> {
    self.oauth2_client.clone().borrow_mut().create_redirect_url(req).await
  }

  async fn sign_in_o_auth2(&self, request: Request<oauth2::ExchangeCodeToTokenRequest>) -> Result<Response<auth::AuthenticationResponse>, Status> {
    let req = request.into_inner();

    let tokens = self.oauth2_client.clone().borrow_mut().exchange_code_to_token(Request::new(oauth2::ExchangeCodeToTokenRequest {
      provider: req.provider.clone(),
      code: req.code,
      csrf_token: req.csrf_token
    })).await?.into_inner();

    let user = self.integrations_client.clone().borrow_mut().get_basic_user(Request::new(integrations::GetBasicUserRequest {
      provider: req.provider.clone(),
      access_token: tokens.access_token.clone()
    })).await?.into_inner();

    let response = self.passport_client.clone().borrow_mut().get_public_user(Request::new(passport::GetPublicUserRequest {
      get_user_by: Some(passport::get_public_user_request::GetUserBy::ProviderId(user.provider_id.clone()))
    })).await;

    let user = match response {
      Ok(r) => Ok(r.into_inner()),
      Err(err) => {
        match err.code() {
          tonic::Code::NotFound => {
            let created_user = self.passport_client.clone().borrow_mut().create_user(Request::new(passport::CreateUserPassportRequest {
              email: user.email,
              username: user.username.clone(),
              password: None,
              provider_id: Some(user.provider_id.clone())
            })).await?.into_inner();

            self.user_client.clone().borrow_mut().create_user(Request::new(user::CreateUserRequest {
              user_id: created_user.user_id.clone(),
              picture: user.picture
            })).await?;

            Ok(passport::GetPublicUserResponse {
              username: user.username,
              user_id: created_user.user_id,
              provider_id: Some(user.provider_id)
            })
          },
          code=> Err(Status::new(code, err.message()))
        }
      }
    }?;

    self.tokens_client.clone().borrow_mut().save_tokens(Request::new(tokens::SaveTokensRequest {
      access_token: tokens.access_token.clone(),
      refresh_token: tokens.refresh_token.clone(),
      user_id: user.user_id.clone(),
      provider: Some(req.provider)
    })).await?;

    let response = Tools::attach_tokens(
      tokens.access_token, tokens.refresh_token,
      auth::AuthenticationResponse {
        username: user.username,
        user_id: user.user_id
      }
    );

    Ok(response)
  }

  async fn edit_user(&self, req: Request<user::EditUserRequest>) -> Result<Response<()>, Status> {
    self.user_client.clone().borrow_mut().edit_user(req).await
  }

  async fn get_user(&self, req: Request<user::GetUserRequest>) -> Result<Response<user::GetUserResponse>, Status> {
    let r =self.user_client.clone().borrow_mut().get_user(req).await?;
    Ok(r)
  }

  async fn get_basic_user(&self, req: Request<user::GetUserRequest>) -> Result<Response<user::GetBasicUserResponse>, Status> {
    let r = self.user_client.clone().borrow_mut().get_basic_user(req).await?;
    Ok(r)
  }

  async fn get_user_friends(&self, req: Request<user::GetUserFriendsRequest>) -> Result<Response<user::GetUserFriendsResponse>, Status> {
    self.user_client.clone().borrow_mut().get_user_friends(req).await
  }

  async fn get_user_followers(&self, req: Request<user::GetUserFollowersRequest>) -> Result<Response<user::GetUserFollowersResponse>, Status> {
    self.user_client.clone().borrow_mut().get_user_followers(req).await
  }

  async fn get_blocked_users(&self, req: Request<user::GetBlockedUsersRequest>) -> Result<Response<user::GetBlockedUsersResponse>, Status> {
    self.user_client.clone().borrow_mut().get_blocked_users(req).await
  }

  async fn follow_user(&self, req: Request<user::UserId>) -> Result<Response<()>, Status> {
    self.user_client.clone().borrow_mut().follow_user(req).await
  }

  async fn unfollow_user(&self, req: Request<user::UserId>) -> Result<Response<()>, Status> {
    self.user_client.clone().borrow_mut().unfollow_user(req).await
  }

  async fn block_user(&self, req: Request<user::UserId>) -> Result<Response<()>, Status> {
    self.user_client.clone().borrow_mut().block_user(req).await
  }

  async fn unblock_user(&self, req: Request<user::UserId>) -> Result<Response<()>, Status> {
    self.user_client.clone().borrow_mut().unblock_user(req).await
  }

  async fn get_user_profile_scopes(&self, req: Request<authorization::GetUserProfileScopesRequest>) -> Result<Response<authorization::GetUserProfileScopesResponse>, Status> {
    self.authorization_client.clone().borrow_mut().get_user_profile_scopes(req).await
  }
}
