use chrono::{Utc, Duration, FixedOffset, DateTime};
use jwt::{AlgorithmType,  Token,  SignWithKey, VerifyWithKey, Header};
use std::collections::BTreeMap;
use std::ops::Add;
use hmac::{Hmac, Mac};

use makoto_logger::error;

const ACCESS_TOKEN_EXPIRATION_TIME_SECS: i64 = 60 * 60; // 1 hour

#[derive(Debug)]
pub enum TokenError {
    Expired,
    Invalid(String), // Additional information
    Internal,
}

impl std::fmt::Display for TokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            TokenError::Internal => "token internal error".to_string(),
            TokenError::Invalid(reason) => reason.to_string(),
            TokenError::Expired => "token is expired".to_string()
        };
        write!(f, "{}", str)
    }
}

enum TokenKind {
    AccessToken(Duration),
    RefreshToken
}

#[derive(Clone)]
pub struct JwtPayload {
    pub user_id: String
}

pub struct Jwt;

impl Jwt {
    pub fn new_access_token(jwt_payload: JwtPayload) -> Result<String, TokenError> {
        let token = Self::sign(
            jwt_payload,
            TokenKind::AccessToken(Duration::seconds(ACCESS_TOKEN_EXPIRATION_TIME_SECS))
        )?;

        Ok(token)
    }

    pub fn new_refresh_token(jwt_payload: JwtPayload) -> Result<String, TokenError> {
        let token = Self::sign(jwt_payload, TokenKind::RefreshToken)?;

        Ok(token)
    }

    pub fn validate_access_token(token: String) -> Result<JwtPayload, TokenError> {
        let token: Token<Header, BTreeMap<String, String>, _>  = token.verify_with_key(&Self::get_jwt_signing_key()).map_err(|err| {
            error!("Error verifying token: {}", err);
            TokenError::Internal
        })?;

        let claims = token.claims();

        // get `expiration_time`
        let exp = match claims.get("exp") {
            Some(exp) => exp,
            None => return Err(TokenError::Invalid("no expiration time in token_claims".to_string()))
        };

        // try to parse to right type
        let exp = match exp.parse::<DateTime<FixedOffset>>() {
            Ok(exp) => exp,
            Err(_) => return Err(TokenError::Internal)
        };

        // get `user_id`
        let user_id = match claims.get("user_id") {
            Some(user_id) => user_id.to_string(),
            None => return Err(TokenError::Invalid("no user_id in token_claims".to_string()))
        };

        // if Now is greater than Exp -> expiration time has exceed
        if exp < Utc::now() {
            return Err(TokenError::Expired);
        };

        Ok(
            JwtPayload {
                user_id,
            }
        )
    }

    fn sign(jwt_payload: JwtPayload, token_kind: TokenKind) -> Result<String, TokenError> {
        let header = Header {
            algorithm: AlgorithmType::Hs256,
            ..Default::default()
        };

        let mut claims = BTreeMap::from([
            ("user_id".to_string(), jwt_payload.user_id),
        ]);

        let mut expiration = Utc::now().fixed_offset();
        if let TokenKind::AccessToken(expiration_time) = token_kind {
            expiration = expiration.add(expiration_time);
            claims.insert("exp".to_string(), expiration.to_string());
        }

        let token = Token::new(header, claims).sign_with_key(&Self::get_jwt_signing_key()).map_err(|err| {
            error!("Error signing token: {}", err);
            TokenError::Internal
        })?;

        Ok(token.as_str().to_string())
    }

    fn get_jwt_signing_key() -> Hmac<sha2::Sha256> {
        let jwt_secret = makoto_config::secrets::Secrets::new().jwt_secret.expect("cannot retrieve jwt secret from env!");

        Hmac::new_from_slice(jwt_secret.as_bytes()).expect("invalid length in generate hmac key! (according to docs xd)")
            //as Hmac<sha2::Sha256>
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;

    fn init() -> JwtPayload {
        let jwt_payload = JwtPayload {
            user_id: "1f2".to_string()
        };

        env::set_var("JWT_SECRET", "jwt_secret_bruh");

        jwt_payload
    }

    #[test]
    fn generate_refresh_jwt() {
        let jwt_payload = init();

        let token = Jwt::new_refresh_token(jwt_payload).unwrap();
        assert_eq!(true, token.len() > 10);
    }

    #[test]
    fn generate_access_jwt() {
        let jwt_payload = init();

        let token = Jwt::new_access_token(jwt_payload).unwrap();
        assert_eq!(true, token.len() > 10);
    }

    #[test]
    fn verify_access_jwt() {
        let jwt_payload = init();

        let token = Jwt::new_access_token(jwt_payload.clone()).unwrap();
        let response = Jwt::validate_access_token(token).unwrap();

        assert_eq!(response.user_id, jwt_payload.user_id);
    }
}