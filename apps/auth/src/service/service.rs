use makoto_grpc::pkg::api_auth::auth_rpc_server::AuthRpc;
use makoto_grpc::pkg::api_auth::*;

use makoto_grpc::Result as GrpcResult;
use makoto_grpc::pkg::general::BoolStatus;
use makoto_logger::{error, info, warn};
use makoto_lib::errors::service::prelude::*;
use makoto_lib::errors::repository::RepositoryError;

use sea_orm::prelude::Uuid;
use tokio::join;
use tonic::{Request as Req, Status, Response, async_trait};

use crate::repository::credentials::{Credentials, UserPayload, GetRecordBy};
use crate::repository::token::{Tokens, GetRecordBy as GetTokenRecordBy, TokenValidationStatus};
use crate::repository::oauth::Oauth;

use crate::utils::hasher::Hasher;
use crate::utils::validator::Validator;

use crate::oauth2::{OAuth2, OAuth2Provider};

pub struct AuthRpcServiceImplementation {
  pub credentials_repository: Credentials,
  pub tokens_repository: Tokens,
  pub oauth_repository: Oauth,

  pub redis: redis::Client,

  pub oauth2: OAuth2
}

impl AuthRpcServiceImplementation {
  pub fn new(credentials_repo: Credentials, tokens_repo: Tokens, oauth_repo: Oauth, redis: redis::Client) -> Self {
    Self {
      credentials_repository: credentials_repo,
      tokens_repository: tokens_repo,
      oauth_repository: oauth_repo,

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
      Validator::username(&req.username).map_err(|msg| Status::invalid_argument(msg))?;
      Validator::email(&req.email).map_err(|msg| Status::invalid_argument(msg))?;
      Validator::password(&req.password).map_err(|msg| Status::invalid_argument(msg))?;
    }

    // check availability
    {
      let (username, email) =  join!(
        self.credentials_repository.is_username_available(&req.username),
        self.credentials_repository.is_email_available(&req.email)
      );

      let is_available = username.map_err(|msg| Status::internal(msg))?;
      if !is_available {
        return Err(Status::already_exists("username is already taken"));
      }

      let is_available = email.map_err(|msg| Status::internal(msg))?;
      if !is_available {
        return Err(Status::already_exists("email is already taken"));
      }
    }

    // new user_id
    let user_id = Uuid::new_v4();

    // create user
    self.credentials_repository.create_user(UserPayload {
      user_id: user_id.clone(),
      username: req.username.clone(),
      email: req.email,
      password: Hasher::hash(req.password).expect("cannot hash password"),
    }).await.map_err(|msg| Status::internal(msg))?;

    // generate tokens
    let (new_access_token, refresh_token) = self.tokens_repository.create_new_token_pair(user_id.clone(), &req.username).await.map_err(|msg| Status::internal(msg))?;

    // initialize empty oauth
    // self.oauth_repository.create_empty_record(user_id.clone()).await.map_err(|msg| Status::internal(msg))?;

    Ok(Response::new(
      AuthenicationServiceResponseWithRefreshToken {
        token: new_access_token,
        username: req.username,
        user_id: user_id.to_string(),
        refresh_token
      }
    ))
  }

  async fn sign_in(&self, req: Req<SignInRequest>) -> GrpcResult<AuthenicationServiceResponseWithRefreshToken> {
    let req = req.into_inner();

    let user = match req.username.is_empty() {
      true => self.credentials_repository.get_user_by_email(&req.email).await,
      false => self.credentials_repository.get_user_by_username(&req.username).await
    }.map_err(|err| Status::not_found(err))?;

    let password = match user.password {
      Some(password) => password,
      None => return Err(Status::not_found("empty password (trying to signin using oauth2 user)"))
    };

    // check password
    if !Hasher::verify(&req.password, &password).map_err(|_| Status::internal("cannot verify password (hasher error)"))? {
      warn!("password is incorrect for user {}", user.username);
      return Err(Status::invalid_argument("password is incorrect"));
    }

    // generate new access_token
    let (new_access_token, refresh_token) = self.tokens_repository.create_new_access_token(user.id.clone(), &user.username)
      .await.map_err(|msg| Status::internal(msg))?;

    Ok(Response::new(
      AuthenicationServiceResponseWithRefreshToken {
        token: new_access_token,
        username: user.username,
        user_id: user.id.to_string(),
        refresh_token: refresh_token

      }
    ))
  }

  async fn refresh(&self, req: Req<RefreshRequest>) -> GrpcResult<AuthenticationServiceResponse> {
    let req = req.into_inner();

    let found_token_model = self.tokens_repository.get_token_record(GetTokenRecordBy::RefreshToken(req.refresh_token)).await.handle()?;

    // should not happen
    let found_user_model = self.credentials_repository.get_user_by_id(found_token_model.user_id).await.map_err(|msg| Status::not_found(msg))?;

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

    let provider = self.oauth2.get_provider_by_name(&req.provider);

    let provider = match provider{
      Some(provider) => provider,
      None => return Err(Status::internal(format!("provider {} not found", req.provider)))
    };

    let (redirect_url, _csrf_token) = provider.create_redirect_url();

    Ok(Response::new(
      OAuth2RedirectUrlResponse {
        url: redirect_url
      }
    ))
  }

  async fn sign_in_oauth(&self, req: Req<SignInOauthRequest>) -> GrpcResult<AuthenicationServiceResponseWithRefreshToken> {
    let req = req.into_inner();
    let provider = self.oauth2.get_provider_by_name(&req.provider);

    let provider = match provider{
      Some(provider) => provider,
      None => return Err(Status::internal(format!("provider {} not found", req.provider)))
    };

    let token = provider.exchange_code_to_token(req.code).await.map_err(|msg| Status::internal(msg))?;

    let user = provider.get_user_by_token(token.access_token.clone()).await.map_err(|msg| Status::internal(msg))?;

    info!("user {} with email {} signed in with oauth2", user.username, user.email);

    todo!()

  }

  async fn sign_in_token(&self, req: Req<SignInTokenRequest>) -> GrpcResult<AuthenticationServiceResponse> {
    let req = req.into_inner();


    let token_validation_status = self.tokens_repository.validate_token_record(req.token.clone()).await;

    match token_validation_status {
      TokenValidationStatus::Expired => return Err(Status::unauthenticated("token is expired, try to refresh it")), // on client, `refresh` endpoint should be called
      TokenValidationStatus::Invalid => return Err(Status::permission_denied("token is invalid")),

      TokenValidationStatus::Valid => ()
    };

    // should not happen
    let found_token_model = self.tokens_repository.get_token_record(GetTokenRecordBy::AccessToken(req.token.clone())).await.handle()?;

    let found_user_model = self.credentials_repository.get_user_by_id(found_token_model.user_id).await.not_found_error()?;

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

    Ok(Response::new(BoolStatus { status: true }))
  }

  async fn confirm_email(&self, req: Req<ConfirmMailRequest>) -> GrpcResult<BoolStatus> {
    todo!()
  }

  async fn update_password(&self, req: Req<SubmitNewPasswordByCodeRequest>) -> GrpcResult<AuthenticationServiceResponse> {
    todo!()
  }

  async fn is_email_available(&self, req: Req<IsEmailAvailableRequest>) -> GrpcResult<BoolStatus> {
    let req = req.into_inner();

    // if `UserModel` is not found -> Err (RepoError::NotFound) would be return
    if let Err(repo_error) = self.credentials_repository.get_user(GetRecordBy::Email(req.email)).await {
      return match repo_error {
        RepositoryError::NotFound(_) => Ok(Response::new(BoolStatus::truthy())),
        _ => Err(Status::internal(repo_error.to_string()))
      };
    }

    // if user was found -> email is not available
    Ok(Response::new(BoolStatus::falsy()))
  }

  async fn is_username_available(&self, req: Req<IsUsernameAvailableRequest>) -> GrpcResult<BoolStatus> {
    let req = req.into_inner();

    // if `UserModel` is not found -> Err (RepoError::NotFound) would be return
    if let Err(repo_error) = self.credentials_repository.get_user(GetRecordBy::Username(req.username)).await {
      return match repo_error {
        RepositoryError::NotFound(_) => Ok(Response::new(BoolStatus::truthy())),
        _ => Err(Status::internal(repo_error.to_string()))
      };
    }

    // if user was found -> username is not available
    Ok(Response::new(BoolStatus::falsy()))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  async fn get_test_service() -> AuthRpcServiceImplementation {
    let db = makoto_db::get_test_db().await;
    let credentials = Credentials::new(db.clone());
    let tokens = Tokens::new(db.clone());
    let oauth = Oauth::new(db.clone());
    let redis_client = redis::Client::open(makoto_config::constants::redis::AUTH_URL).unwrap();

    AuthRpcServiceImplementation::new(credentials, tokens, oauth, redis_client)
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
