use chrono::{Utc, Duration, FixedOffset, DateTime};
use jwt::{AlgorithmType,  Token,  SignWithKey, VerifyWithKey, Header};
use sea_orm::prelude::{DateTimeWithTimeZone, DateTimeUtc};
use std::{collections::BTreeMap, ops::Sub};
use std::ops::Add;
use hmac::{Hmac, Mac};

use makoto_logger::error;

const ACCESS_TOKEN_EXPIRATION_TIME_SECS: i64 = 60 * 60; // 1 hour

pub enum TokenValidationError {
  Expired,
  Invalid(String), // Additional information
  Internal
}

enum TokenKind {
  AccessToken(Duration),
  RefreshToken
}

enum TokenCreated {
  AccessToken(String, DateTimeWithTimeZone),
  RefreshToken(String)
}

#[derive(Clone)]
pub struct JwtPayload {
  pub username: String,
  pub user_id: String
}

pub struct Jwt;

impl Jwt {
  pub fn new_access_token(jwt_payload: JwtPayload) -> Result<(String, DateTimeWithTimeZone), String> {
    let token = Self::sign(
      jwt_payload, TokenKind::AccessToken(
      Duration::seconds(ACCESS_TOKEN_EXPIRATION_TIME_SECS))
    ).map_err(|err| err.to_string())?;

    if let TokenCreated::AccessToken(token, exp) = token {
      Ok((token, exp))
    } else {
      Err("Error creating access token".to_string()) // should not happen
    }
  }

  pub fn new_refresh_token(jwt_payload: JwtPayload) -> Result<String, String> {
    let token = Self::sign(jwt_payload, TokenKind::RefreshToken).map_err(|err| err.to_string())?;
    if let TokenCreated::RefreshToken(token) = token {
      Ok(token)
    } else {
      Err("Error creating refresh token".to_string()) // should not happen
    }
  }

  pub fn validate_access_token(token: String) -> Result<JwtPayload, TokenValidationError> {
    let token: Token<Header, BTreeMap<String, String>, _>  = token.verify_with_key(&Self::get_jwt_signing_key()).map_err(|err| {
      error!("Error verifying token: {}", err);
      TokenValidationError::Internal
    })?;

    let claims = token.claims();

    // get `expiration_time`
    let exp = match claims.get("exp") {
      Some(exp) => exp,
      None => return Err(TokenValidationError::Invalid("no expiration time in token_claims".to_string()))
    };

    // try to parse to right type
    let exp = match exp.parse::<DateTime<FixedOffset>>() {
      Ok(exp) => exp,
      Err(_) => return Err(TokenValidationError::Internal)
    };

    // get `user_id`
    let user_id = match claims.get("user_id") {
      Some(user_id) => user_id.to_string(),
      None => return Err(TokenValidationError::Invalid("no user_id in token_claims".to_string()))
    };

    // get `username`
    let username = match claims.get("username") {
      Some(username) => username.to_string(),
      None => return Err(TokenValidationError::Invalid("no username in token_claims".to_string()))
    };

    // if Now is greater than Exp -> expiration time has exceed
    if exp < Utc::now() {
      return Err(TokenValidationError::Expired);
    };

    Ok(
      JwtPayload {
        user_id,
        username,
      }
    )
  }


  fn sign(jwt_payload: JwtPayload, token_kind: TokenKind) -> Result<TokenCreated, String> {
    let header = Header {
      algorithm: AlgorithmType::Hs256,
      ..Default::default()
    };

    let mut claims = BTreeMap::from([
      ("username".to_string(), jwt_payload.username),
      ("user_id".to_string(), jwt_payload.user_id),
    ]);

    let mut expiration = Utc::now().fixed_offset();
    if let TokenKind::AccessToken(expiration_time) = token_kind {
      expiration = expiration.add(expiration_time);
      claims.insert("exp".to_string(), expiration.to_string());
    }

    let token = Token::new(header, claims).sign_with_key(&Self::get_jwt_signing_key()).map_err(|err| {
      error!("Error signing token: {}", err);
      err.to_string()
    })?;

    let token = token.as_str().to_string();

    match token_kind {
      TokenKind::AccessToken(_) => Ok(TokenCreated::AccessToken(token, expiration)),
      TokenKind::RefreshToken => Ok(TokenCreated::RefreshToken(token)),
    }

  }

  fn get_jwt_signing_key() -> Hmac<sha2::Sha256> {
      let jwt_secret = makoto_config::secrets::Secrets::new().jwt_secret.expect("cannot retrieve jwt secret from env!");

      Hmac::new_from_slice(jwt_secret.as_bytes()).expect("invalid length in generate hmac key! (according to docs)") as Hmac<sha2::Sha256>
  }
}

#[cfg(test)]
mod tests {
  use std::env;

use super::*;

  fn prelude() -> JwtPayload {
    let jwt_payload = JwtPayload {
      username: "dehwyy".to_string(),
      user_id: "1f2".to_string()
    };

    env::set_var("JWT_SECRET", "jwt_secret_bruh");

    jwt_payload
  }

  #[test]
  fn generate_refresh_jwt() {
    let jwt_payload = prelude();

    let token = Jwt::new_refresh_token(jwt_payload).unwrap();
    assert_eq!(true, token.len() > 10);
  }

  #[test]
  fn generate_access_jwt() {
    let jwt_payload = prelude();

    let token = Jwt::new_access_token(jwt_payload).unwrap();
    assert_eq!(true, token.0.len() > 10);
  }

  #[test]
  fn verify_access_jwt() {
    let jwt_payload = prelude();

    let token = Jwt::new_access_token(jwt_payload).unwrap();
    let response = Jwt::validate_access_token(token.0).map_err(|e| {
      "invalid token"
    }).unwrap();

    assert_eq!(response.username, "dehwyy".to_string());
  }

}
