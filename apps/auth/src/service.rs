use std::borrow::BorrowMut;
use rand::Rng;
use tonic::{Status, Response, Request};
use uuid::Uuid;

use makoto_grpc::metadata::{UserRole,METADATA_USER_ROLE_KEY};
use makoto_grpc::pkg::auth as rpc;
use makoto_grpc::pkg::{tokens, passport, mailer, user};
use makoto_grpc::pkg::general::IsOkResponse;
use makoto_grpc::errors::GrpcHandleError;
use makoto_grpc::pkg::auth::{AuthenticationServiceResponse, SendEmailVerificationCodeRequest, VerifyEmailCodeRequest};

use mafuyu_lib::errors::prelude::*;

use crate::repository::credentials::{Credentials, GetCredentialsRecordBy as GetUserRecordBy};
use crate::repository::tokens::{Tokens, GetTokenRecordBy};
use crate::utils::hasher::Hasher;
use crate::utils::validator::Validator;
use crate::utils::redis_ops::RedisOperations;

pub struct AuthRpcServiceImplementation<T = tonic::transport::Channel> {
  credentials_repo: Credentials,
  tokens_repo: Tokens,
  tokens_client: tokens::tokens_rpc_client::TokensRpcClient<T>,
  passport_client: passport::passport_rpc_client::PassportRpcClient<T>,
  user_client: user::user_rpc_client::UserRpcClient<T>,
  mailer_client: mailer::mailer_rpc_client::MailerRpcClient<T>,
  redis_operations: RedisOperations
}

impl AuthRpcServiceImplementation {
  pub async fn new(credentials: Credentials, tokens: Tokens, redis_client: redis::Client) -> Self {
    let clients = makoto_grpc::RpcClients::get_all_client().await;

    Self {
      credentials_repo: credentials,
      tokens_repo: tokens,
      tokens_client: clients.tokens_client.unwrap(),
      passport_client: clients.passport_client.unwrap(),
      user_client: clients.user_client.unwrap(),
      mailer_client: clients.mailer_client.unwrap(),
      redis_operations: RedisOperations::new(redis_client)
    }
  }
}

#[tonic::async_trait]
impl rpc::auth_rpc_server::AuthRpc for AuthRpcServiceImplementation {
  async fn sign_up(&self, req: Request<rpc::SignUpRequest>) -> Result<Response<AuthenticationServiceResponse>, Status> {
    let req = req.into_inner();

    // validation
    {
      Validator::username(&req.username).invalid_argument_error()?;
      Validator::email(&req.email).invalid_argument_error()?;
      Validator::password(&req.password).invalid_argument_error()?;
    }

    // check availability
    {
      let (username, email) =  tokio::join!(
        self.credentials_repo.get_user(GetUserRecordBy::Username(req.username.clone())),
        self.credentials_repo.get_user(GetUserRecordBy::Email(req.email.clone()))
      );

      match username {
        Ok(_) => return Err(Status::already_exists("username is already taken")),
        Err(RepositoryError::NotFound(_)) => {}
        Err(err) => return Err(Status::internal(err.to_string())),
      }

      match email {
        Ok(_) => return Err(Status::already_exists("email is already taken")),
        Err(RepositoryError::NotFound(_)) => {}
        Err(err) => return Err(Status::internal(err.to_string())),
      }
    }

    let user_password = Hasher::hash(req.password).invalid_argument_error()?;

    // create
    let created_user_passport = self.passport_client.clone().borrow_mut().create_user(Request::new(passport::CreateUserPassportRequest {
      username: req.username.clone(),
      email: Some(req.email),
      password: Some(user_password),
      provider_id: None
    })).await?.into_inner();

    let mut user_client_clone = self.user_client.clone();
    let created_user_future = user_client_clone.borrow_mut().create_user(Request::new(user::CreateUserRequest {
      user_id: created_user_passport.user_id.clone(),
      picture: None
    }));

    let tokens = self.tokens_client.clone().borrow_mut().generate_token_pair(Request::new(tokens::GenerateTokenPairRequest {
      user_id: created_user_passport.user_id.clone()
    })).await?.into_inner();

    created_user_future.await?;

    Ok(Response::new(
      AuthenticationServiceResponse {
        access_token: tokens.access_token,
        refresh_token: tokens.refresh_token,
        user_id: created_user_passport.user_id,
        username: req.username
      }
    ))
  }

  async fn sign_in(&self, req: Request<rpc::SignInRequest>) -> Result<Response<AuthenticationServiceResponse>, Status> {
    let req = req.into_inner();

    let user = match req.login.unwrap_or_internal("cannot get `login` field from request")? {
      rpc::sign_in_request::Login::Email(email) => {
        Validator::email(&email).invalid_argument_error()?;

        self.credentials_repo.get_user(GetUserRecordBy::Email(email)).await
      },
      rpc::sign_in_request::Login::Username(username) => {
        Validator::username(&username).invalid_argument_error()?;

        self.credentials_repo.get_user(GetUserRecordBy::Username(username.clone())).await
      },
    }.handle()?;

    let password = user.password.unwrap_or_not_found("no password on user_model")?;

    // check password
    if !Hasher::verify(&req.password, &password).invalid_argument_error()? {
      return Err(Status::unauthenticated("invalid credentials"));
    }

    // new tokens pair (actually only new access_token)
    let tokens = self.tokens_client.clone().borrow_mut().generate_token_pair(Request::new(tokens::GenerateTokenPairRequest {
      user_id: user.id.to_string().clone()
    })).await?.into_inner();

    Ok(Response::new(
      AuthenticationServiceResponse {
        username: user.username,
        user_id: user.id.to_string(),
        access_token: tokens.access_token,
        refresh_token: tokens.refresh_token
      }
    ))
  }

  async fn sign_in_with_token(&self, req: Request<rpc::SignInWithTokenRequest>) -> Result<Response<AuthenticationServiceResponse>, Status> {
    let req = req.into_inner();

    let response = self.tokens_client.clone().borrow_mut().validate_token(Request::new(tokens::ValidateTokenRequest {
      access_token: req.token.clone(),
      provider: None // we don't know, let the service infer it itself
    })).await?.into_inner();

    let user_id = match response.user_id {
      Some(id) => id,
      None => {
        self.tokens_repo.get_token_record(GetTokenRecordBy::AccessToken(req.token.clone()))
            .await.handle()?.user_id.to_string()
      }
    };

    let user_id = Uuid::try_parse(&user_id).map_err(|err| err.to_string()).invalid_argument_error()?;

    let token_model = self.tokens_repo.get_token_record(GetTokenRecordBy::UserId(user_id.clone())).await.handle()?;
    let user = self.credentials_repo.get_user(GetUserRecordBy::UserId(user_id)).await.handle()?;

    let mut r = Response::new(
      AuthenticationServiceResponse {
        access_token: req.token,
        refresh_token: token_model.refresh_token.unwrap_or_default(),
        user_id: user.id.into(),
        username: user.username
      }
    );

    r.metadata_mut().insert(METADATA_USER_ROLE_KEY, UserRole::from(user.role).to_string().parse().invalid_argument_error()?);

    Ok(r)
  }

  async fn sign_out(&self, req: Request<rpc::SignOutRequest>) -> Result<Response<IsOkResponse>, Status> {
    let req = req.into_inner();

    let token_model = self.tokens_repo.get_token_record(GetTokenRecordBy::AccessToken(req.access_token.clone())).await.handle()?;

    let is_ok_response = self.tokens_client.clone().borrow_mut().clear_tokens(Request::new(tokens::ClearTokensRequest {
      user_id: token_model.user_id.into(),
      access_token: Some(req.access_token)
    })).await;

    is_ok_response
  }

  async fn send_email_verification_code(&self, req: Request<SendEmailVerificationCodeRequest>) -> Result<Response<()>, Status> {
    let req = req.into_inner();

    let code =  rand::thread_rng().gen_range(100000..=999999).to_string();

    self.mailer_client.clone().borrow_mut().send_confirmation_email(mailer::SendConfirmationEmailRequest {
      subject: "Email verification.".to_string(),
      to: req.email.clone(),
      code: code.clone()
    }).await?;

    self.redis_operations.set_email_code(&req.email, &code).await.internal_error()?;

    Ok(Response::new(()))
  }

  async fn verify_email_code(&self, req: Request<VerifyEmailCodeRequest>) -> Result<Response<IsOkResponse>, Status> {
    let req = req.into_inner();

    let code = self.redis_operations.get_email_code(&req.email).await.internal_error()?;

    Ok(Response::new(IsOkResponse {
      is_ok: code == req.code
    }))
  }
}