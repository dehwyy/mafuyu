use makoto_grpc::pkg::api_auth::auth_rpc_server::AuthRpc;
use makoto_grpc::pkg::api_auth::*;

use makoto_grpc::Result as GrpcResult;
use makoto_grpc::pkg::general::BoolStatus;
use makoto_logger::{error, info, warn};
use makoto_lib::errors::service::prelude::*;
use makoto_lib::errors::{repository::RepositoryError};

use sea_orm::prelude::Uuid;
use tokio::join;
use tonic::{Request as Req, Status, Response, async_trait};

use crate::repository::credentials::{Credentials, UserPayload, GetRecordBy};
use crate::repository::token::{Tokens, GetRecordBy as GetTokenRecordBy, TokenValidationStatus, CreateOAuth2TokenRecordPayload};

use crate::utils::hasher::Hasher;
use crate::utils::validator::Validator;

use crate::oauth2::{OAuth2, OAuth2Provider, OAuth2ProviderName};

pub struct AuthRpcServiceImplementation {
  pub credentials_repository: Credentials,
  pub tokens_repository: Tokens,

  pub redis: redis::Client,

  pub oauth2: OAuth2
}

impl AuthRpcServiceImplementation {
  pub fn new(credentials_repo: Credentials, tokens_repo: Tokens, redis: redis::Client) -> Self {
    Self {
      credentials_repository: credentials_repo,
      tokens_repository: tokens_repo,

      redis,

      oauth2: OAuth2::new()
    }
  }
}

#[async_trait]
impl AuthRpc for AuthRpcServiceImplementation {
  async fn sign_up(&self, req: Req<SignUpRequest>) -> GrpcResult<AuthenicationServiceResponseWithRefreshToken> {
    let req = req.into_inner();

    // validation
    {
      Validator::username(&req.username).invalid_argument_error()?;
      Validator::email(&req.email).invalid_argument_error()?;
      Validator::password(&req.password).invalid_argument_error()?;
    }

    // check availability
    {
      let (username, email) =  join!(
        self.credentials_repository.get_user(GetRecordBy::Username(req.username.clone())),
        self.credentials_repository.get_user(GetRecordBy::Email(req.email.clone()))
      );

      if !username.is_not_found() {
        return Err(Status::already_exists("username is already taken"));
      }

      if !email.is_not_found() {
        return Err(Status::already_exists("email is already taken"));
      }
    }


    // create user
    let record = self.credentials_repository.create_user(UserPayload {
      user_id: Uuid::new_v4(),
      username: req.username.clone(),
      email: Some(req.email),
      password: Some(Hasher::hash(req.password).expect("cannot hash password")),
    }).await.handle()?;

    // generate tokens
    let (new_access_token, refresh_token) = self.tokens_repository.create_new_token_pair(record.id, &req.username)
        .await.internal_error()?;

    // initialize empty oauth
    // self.oauth_repository.create_empty_record(user_id.clone()).await.map_err(|msg| Status::internal(msg))?;

    Ok(Response::new(
      AuthenicationServiceResponseWithRefreshToken {
        token: new_access_token,
        username: req.username,
        user_id: record.id.to_string(),
        refresh_token
      }
    ))
  }

  async fn sign_in(&self, req: Req<SignInRequest>) -> GrpcResult<AuthenicationServiceResponseWithRefreshToken> {
    let req = req.into_inner();

    let user = match req.username.is_empty() {
      true => self.credentials_repository.get_user(GetRecordBy::Email(req.email.clone())).await,
      false => self.credentials_repository.get_user(GetRecordBy::Username(req.username.clone())).await
    }.handle()?;;

     let password = user.password.unwrap_or_status("trying to auth via oauth2 account")?;

    // check password
    Hasher::verify(&req.password, &password).invalid_argument_error()?;

    // generate new access_token
    let (new_access_token, refresh_token) = self.tokens_repository.create_new_access_token(user.id.clone(), &user.username)
      .await.internal_error()?;

    Ok(Response::new(
      AuthenicationServiceResponseWithRefreshToken {
        token: new_access_token,
        username: user.username,
        user_id: user.id.to_string(),
        refresh_token
      }
    ))
  }

  async fn refresh(&self, req: Req<RefreshRequest>) -> GrpcResult<AuthenticationServiceResponse> {
    let req = req.into_inner();

    let found_token_model = self.tokens_repository.get_token_record(GetTokenRecordBy::RefreshToken(req.refresh_token)).await.handle()?;

    let found_user_model = self.credentials_repository.get_user(GetRecordBy::UserId(found_token_model.user_id))
        .await.handle()?;

    // removes all invalid tokens | create new access token
    let (clear_token, create_token) = tokio::join!(
      self.tokens_repository.clear_invalid_tokens(found_token_model.clone()),
      self.tokens_repository.create_new_access_token(found_token_model.user_id, &found_user_model.username)
    );

    clear_token.handle()?;
    let (new_access_token, _refresh) = create_token.internal_error()?;

    Ok(Response::new(
      AuthenticationServiceResponse {
        token: new_access_token,
        user_id: found_user_model.id.into(),
        username: found_user_model.username
      }
    ))
  }

  async fn create_o_auth2_redirect_url(&self, req: Req<OAuth2RedirectUrlRequest>) -> GrpcResult<OAuth2RedirectUrlResponse> {
    let req = req.into_inner();

    let provider_name = OAuth2ProviderName::from_str(&req.provider)
        .unwrap_or_status(&format!("provider {provider} wasn't found!", provider=req.provider))?;

    let provider = self.oauth2.get_provider(provider_name);

    let (redirect_url, _csrf_token) = provider.create_redirect_url();

    Ok(Response::new(
      OAuth2RedirectUrlResponse {
        url: redirect_url
      }
    ))
  }

  async fn sign_in_oauth(&self, req: Req<SignInOauthRequest>) -> GrpcResult<AuthenicationServiceResponseWithRefreshToken> {
    let req = req.into_inner();

    let provider_name = OAuth2ProviderName::from_str(&req.provider).unwrap_or_status("provider not found")?;

    let provider = self.oauth2.get_provider(provider_name);

    let token = provider.exchange_code_to_token(req.code).await.map_err(|msg| {
      Status::internal(msg)
    })?;

    let user = provider.get_user_by_token(token.access_token.clone()).await.internal_error()?;

    let db_user = self.credentials_repository.get_user(GetRecordBy::Username(user.username.clone())).await;

    // user already in db
    if let Ok(user) = db_user {
      return Ok(Response::new(
        AuthenicationServiceResponseWithRefreshToken {
          token: token.access_token,
          refresh_token: token.refresh_token.unwrap_or_default(),
          user_id: user.id.into(),
          username: user.username
        }
      ));
    }

    let created_user = match db_user.err().unwrap() {
      RepositoryError::DbError(err) => Err(err).internal_error(),
      RepositoryError::NotFound(_) => {
        let record = self.credentials_repository.create_user(UserPayload {
          user_id: Uuid::new_v4(),
          username: user.username,
          email: user.email,
          password: None
        }).await.handle()?;

        self.tokens_repository.create_oauth2_record(CreateOAuth2TokenRecordPayload {
          provider:  req.provider,
          user_id: record.id,
          access_token: token.access_token.clone(),
          refresh_token: token.refresh_token.clone()
        }).await.handle()?;

        Ok(record)
      }
    }?;

    Ok(Response::new(
      AuthenicationServiceResponseWithRefreshToken {
        token: token.access_token,
        refresh_token: token.refresh_token.unwrap_or_default(),
        user_id: created_user.id.into(),
        username: created_user.username
      }
    ))
  }

  async fn sign_in_token(&self, req: Req<SignInTokenRequest>) -> GrpcResult<AuthenticationServiceResponse> {
    let req = req.into_inner();

    let found_token_model = self.tokens_repository.get_token_record(GetTokenRecordBy::AccessToken(req.token.clone())).await.handle()?;

    // Some => OAuth2 token
    // None => `native` token
    let oauth2_provider = OAuth2ProviderName::from_str(&found_token_model.provider);

    // Perform validation
    match oauth2_provider {
      // oauth2 flow
      Some(provider_name) => {
        let provider = self.oauth2.get_provider(provider_name);

        // Perform request. If it fails -> error is returned
        if let Err(err) = provider.get_user_by_token(req.token.clone()).await {
          return Err(Status::unauthenticated(err))
        };
      }
      // native token flow
      None => {
        let token_validation_status = self.tokens_repository.validate_token_record(req.token.clone()).await;

        match token_validation_status {
          TokenValidationStatus::Expired => return Err(Status::unauthenticated("token is expired, try to refresh it")), // on client, `refresh` endpoint should be called
          TokenValidationStatus::Invalid => return Err(Status::permission_denied("token is invalid")),

          TokenValidationStatus::Valid => ()
        }
      }
    };

    // Should be found, I guess
    let found_user_model = self.credentials_repository.get_user(GetRecordBy::UserId(found_token_model.user_id)).await.handle()?;;

    Ok(Response::new(
      AuthenticationServiceResponse {
        token: req.token,
        user_id: found_user_model.id.into(),
        username: found_user_model.username
      }
    ))

  }

  async fn sign_out(&self, req: Req<SignOutRequest>) -> GrpcResult<BoolStatus> {
    let req = req.into_inner();


    match self.tokens_repository.clear_access_token(req.token).await {
      Ok(_) => Ok(()),
      Err(err) => {
        error!("[clear_access_token]: {}", err.to_string());

        match err {
          RepositoryError::NotFound(msg) => Err(Status::not_found(msg)),
          _ => Err(Status::internal(err.to_string()))
        }
      }
    }?;

    Ok(Response::new(BoolStatus::truthy()))
  }

  async fn confirm_email(&self, req: Req<ConfirmMailRequest>) -> GrpcResult<BoolStatus> {
    todo!()
  }

  async fn update_password(&self, req: Req<SubmitNewPasswordByCodeRequest>) -> GrpcResult<AuthenticationServiceResponse> {
    todo!()
  }

  async fn is_email_available(&self, req: Req<IsEmailAvailableRequest>) -> GrpcResult<BoolStatus> {
    let req = req.into_inner();

    Ok(Response::new(BoolStatus {
      status: self.credentials_repository.get_user(GetRecordBy::Email(req.email)).await.is_not_found()
    }))
  }

  async fn is_username_available(&self, req: Req<IsUsernameAvailableRequest>) -> GrpcResult<BoolStatus> {
    let req = req.into_inner();

    Ok(Response::new(BoolStatus {
      status: self.credentials_repository.get_user(GetRecordBy::Username(req.username)).await.is_not_found()
    }))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  async fn get_test_service() -> AuthRpcServiceImplementation {
    let db = makoto_db::get_test_db().await;
    let credentials = Credentials::new(db.clone());
    let tokens = Tokens::new(db.clone());
    let redis_client = redis::Client::open(makoto_config::constants::redis::AUTH_URL).unwrap();

    AuthRpcServiceImplementation::new(credentials, tokens, redis_client)
  }


  #[tokio::test]
  async fn test_signup_signin_flow() {
    let service = get_test_service().await;

    service.sign_up(Req::new(SignUpRequest {
      username: "dehwyy".to_string(),
      email: "dehwyy@qqq.com".to_string(),
      password: "some_pass".to_string()
    }))
      .await.map_err(|err| {
        eprintln!("{err:?}")
      })
      .unwrap();

      service.sign_in(Req::new(SignInRequest {
        username: "dehwyy".to_string(),
        email: "dehwyy@qqq.com".to_string(),
        password: "some_pass".to_string()
      }))
        .await.map_err(|err| {
          eprintln!("{err:?}")
        })
        .unwrap();
  }
}
