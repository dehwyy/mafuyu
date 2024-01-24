use std::future::Future;
use sea_orm::prelude::Expr;
use sea_orm::{DatabaseConnection, ActiveModelTrait, ColumnTrait, QueryFilter, EntityTrait, QueryTrait};
use uuid::Uuid;

use makoto_db::models::user_tokens::{self, Entity as UserTokens};
use makoto_db::utilities::*;
use makoto_lib::errors::repository::prelude::*;
use makoto_lib::errors::repository::RepositoryError;
use crate::oauth2::{OAuth2, OAuth2ProviderName, OAuth2Provider};

use crate::utils::jwt::{Jwt, JwtPayload, TokenValidationError};

pub enum TokenValidationStatus {
  Valid,
  Expired,
  Invalid
}

pub enum GetRecordBy {
  UserId(Uuid),
  AccessToken(String),
  RefreshToken(String)
}

pub struct CreateOAuth2TokenRecordPayload {
  pub user_id: Uuid,
  pub provider: String,
  pub access_token: String,
  pub refresh_token: Option<String>,
}

pub struct Tokens {
  db: DatabaseConnection,
  oauth2: OAuth2
}

impl Tokens {
  pub fn new(db: DatabaseConnection) -> Self {
    Self {
      db,
      oauth2: OAuth2::new()
    }
  }

  pub async fn create_oauth2_record(&self, payload: CreateOAuth2TokenRecordPayload) -> Result<user_tokens::Model, RepositoryError> {
    let token_model = user_tokens::ActiveModel {
      user_id: not_null(payload.user_id),
      provider: not_null(payload.provider),
      access_token: nullable(vec!(payload.access_token)),
      refresh_token: not_null(payload.refresh_token),
      ..Default::default()
    };

    token_model.insert(&self.db).await.handle()
  }

  pub async fn create_new_token_pair(&self, user_id: Uuid, username: &str) -> Result<(String, String), String> {

    let payload = JwtPayload {
      user_id: user_id.to_string(),
      username: username.to_string()
    };

    let new_access_token = Jwt::new_access_token(payload.clone())?;
    let new_refresh_token = Jwt::new_refresh_token(payload)?;

    let new_token_model = user_tokens::ActiveModel {
      access_token: nullable(vec!(new_access_token.0.clone())),
      refresh_token: nullable(new_refresh_token.clone()),
      user_id: not_null(user_id),
      ..Default::default()
    };
    new_token_model.insert(&self.db).await.map_err(|err| {
        err.to_string()
    })?;

    Ok((new_access_token.0, new_refresh_token))
  }

  /// Returns `(new_access_token, refresh_token)`
  pub async fn create_new_access_token(&self, user_id: Uuid, username: &str) -> Result<(String, String), String> {

    let payload = JwtPayload {
      user_id: user_id.to_string(),
      username: username.to_string()
    };

    let token_record = self.get_token_record(GetRecordBy::UserId(user_id.clone())).await.map_err(|msg| msg.to_string())?;
    let refresh_token = token_record.clone().refresh_token;

    let mut token_record: user_tokens::ActiveModel = token_record.into();
    let (new_access_token, expiry) = Jwt::new_access_token(payload)?;

    // get all previous token (safe operations)
    let mut old_tokens = token_record.access_token.take().unwrap_or_default().unwrap_or_default();

    old_tokens.push(new_access_token.clone());

    token_record.access_token = nullable(old_tokens);

    token_record.update(&self.db).await.map_err(|err| err.to_string())?;

    Ok((new_access_token, refresh_token.expect("TODO! 101 line"))) // todo
  }

  pub async fn validate_token_record(&self, access_token: String) -> TokenValidationStatus {
    let validation_result= Jwt::validate_access_token(access_token);

    match validation_result {
      Ok(_) => TokenValidationStatus::Valid,
      Err(validation_error) => {
        match validation_error {
          TokenValidationError::Expired => TokenValidationStatus::Expired,
          _ => TokenValidationStatus::Invalid
        }
      }
    }
  }

  /// Removes all invalid tokens from PostgresArray
  pub async fn clear_invalid_tokens(&self, token_record: user_tokens::Model) -> Result<(), RepositoryError> {

    let oauth2_provider = OAuth2ProviderName::from_str(&token_record.provider);
    let mut token_record: user_tokens::ActiveModel = token_record.into();

    let all_access_tokens = token_record.access_token.take().unwrap_or_default().unwrap_or_default();

    let filtered_access_tokens = match oauth2_provider {
      Some(provider_name) => {
        let provider = self.oauth2.get_provider(provider_name);
        let mut access_tokens: Vec<String> = vec!();

        for token in all_access_tokens {
          if let Ok(_) = provider.get_user_by_token(token.clone()).await {
            access_tokens.push(token);
          }
        }

        access_tokens
      },
      None => {
        all_access_tokens.iter().filter_map(|token| {
          if let Err(_) = Jwt::validate_access_token(token.to_string()) {
            return None;
          }

          Some(token.to_string())
        }).collect::<_>()
      }
    };

    token_record.access_token = nullable(filtered_access_tokens);

    token_record.update(&self.db).await.handle()?;

    Ok(())
  }

  /// Acts as Cleaner of tokens (removes from db invalid tokens (expired tokens) and invalidate requested token)
  pub async fn clear_access_token(&self, access_token: String) -> Result<user_tokens::Model, RepositoryError> {

    let token_record = UserTokens::find()
      .filter(user_tokens::Column::AccessToken.eq(access_token.clone()))
      .one(&self.db)
      .await.handle()?.extract("token not found")?;

    let oauth2_provider = OAuth2ProviderName::from_str(&token_record.provider);
    let mut token_record: user_tokens::ActiveModel = token_record.into();

    let all_access_tokens = token_record.access_token.take().unwrap_or_default().unwrap_or_default();
    let filtered_access_tokens = match oauth2_provider {
      Some(provider_name) => {
        let provider = self.oauth2.get_provider(provider_name);
        let mut access_tokens: Vec<String> = vec!();

        for token in all_access_tokens {
          if token == access_token {
            continue
          }

          if let Ok(_) = provider.get_user_by_token(token.clone()).await {
            access_tokens.push(token);
          }
        }

        access_tokens
      },
      None => {
        all_access_tokens.iter().filter_map(|token| {
          if token == &access_token {
            return None;
          }

          if let Err(_) = Jwt::validate_access_token(token.to_string()) {
            return None;
          }

          Some(token.to_string())
        }).collect::<_>()
      }
    };

    token_record.access_token = nullable(filtered_access_tokens);

    let token_record = token_record.update(&self.db).await.handle()?;

    Ok(token_record)
  }

  pub async fn get_token_record(&self, get_by: GetRecordBy) -> Result<user_tokens::Model, RepositoryError> {
    let filter = match get_by {
      GetRecordBy::UserId(id) => user_tokens::Column::UserId.eq(id),
      // I was struggling in about 2 hour to make this query...
      GetRecordBy::AccessToken(token) => Expr::cust(format!(
        r#"'{token}' = ANY("{user_tokens}"."access_token")"#,
        token=token,
        user_tokens=user_tokens::Column::AccessToken.entity_name().to_string(),
      )),
      GetRecordBy::RefreshToken(token) => user_tokens::Column::RefreshToken.eq(token)
    };

    let token_record = UserTokens::find()
      .filter(filter)
      .one(&self.db)
      .await.handle()?.extract("token not found")?;

    Ok(token_record)
  }
}
