use async_nats::Message;
use sea_orm::IntoActiveValue;
use serde_json::to_string as to_json;
use makoto_logger::{error, info};
use makoto_lib::errors::{HandleRepositoryError, HandleError};
use uuid::Uuid;
use crate::internal::jwt::{Jwt, JwtPayload, TokenError};
use crate::internal::repo::{GetTokenRecordBy, Repo as TokenRepo};
use crate::router::RouterError;
use crate::router::{payload::*, response::*};
use makoto_db::models::user_tokens;

type RouteReturn =Result<serde_json::Result<String>, RouterError>;

pub struct Router {
    client: async_nats::Client,
    repo: TokenRepo
}

impl Router {
    pub fn new(nats_client: async_nats::Client, token_repo: TokenRepo) -> Self {
        Self {
            client: nats_client,
            repo: token_repo
        }
    }

    pub async fn handle(&self, subject: &str, message: Message) {

        let reply_subject = message.reply.clone();

        let call = match subject {
            "validate_token" => self.call_validate_token(message).await,
            "make_tokens"    => self.call_make_tokens_pair(message).await,
            _ => {
                error!("unknown subject {subject}", subject=&message.subject);

                Err(RouterError::Internal(
                    format!("[unknown subject {subject}", subject=&message.subject)
                ))
            }
        };

        if let Some(reply_subject) = reply_subject {

            let message_to_respond = match call {
                Ok(v) => CommonResponse {
                    data: v.unwrap_or_default(),
                    error: None
                },
                Err(e) =>  CommonResponse {
                    data: "".to_string(),
                    error: Some(e.to_string())
                }
            };

            let message_to_respond = to_json(&message_to_respond).expect("cannot serialize json");

            if let Err(err) = self.client.publish(reply_subject, message_to_respond.into()).await {
                error!("cannot reply {err}");
            }
        }

    }

    /// ### Perform `native` token validation, @see (`docs/about-authentication.md`)
    /// * @payload  ( `JSON { access_token: String } ` );
    /// ^ @responds ( `JSON { access_token: String, refresh_token: String } `);
    async fn call_validate_token(&self, message: Message) -> RouteReturn {

        let payload = get_payload::<TokenValidationPayload>(&message.payload)?;

        let _v = Jwt::validate_access_token(payload.access_token).map_err(|err| {
            RouterError::Local(err.to_string())
        })?;

        Ok(to_json(&TokenValidationResponse{
            valid: true
        }))
    }

    /// ### Creates/Refreshes/Gets token pair (depends on access_token state)
    /// * @payload  ( `JSON { user_id: String (Uuid), provider: Option<String> }` );
    /// * @responds ( `JSON { access_token: String, refresh_token: String } ` );
    async fn call_make_tokens_pair(&self, message: Message) -> RouteReturn {
        let payload = get_payload::<MakeTokensPayload>(&message.payload)?;

        let user_id = Uuid::try_parse(&payload.user_id).map_err(|err| {
            RouterError::WrongPayload(err.to_string())
        })?;

        let record = self.repo.get_token_record(GetTokenRecordBy::UserId(user_id)).await;

        let jwt_payload = JwtPayload {
            user_id: payload.user_id.clone()
        };

        let access_token = Jwt::new_access_token(jwt_payload.clone()).map_err(|err| {
            RouterError::Internal(err.to_string())
        })?;

        let refresh_token = match record.is_not_found(){
            true => Jwt::new_refresh_token(jwt_payload),
            // Both `record` and `refresh_token` should be defined as as record is !!found => found, and token provider
            false => match record.unwrap().refresh_token {
                Some(token) => Ok(token),
                None => Err(TokenError::Internal)
            }
        }.map_err(|err| {
           RouterError::Internal(err.to_string())
        })?;


        self.repo.insert_tokens(user_id, &access_token, &refresh_token).await.map_err(|err| {
            RouterError::Internal(err.to_string())
        })?;

        Ok(to_json(&MakeTokensResponse {
            access_token,
            refresh_token
        }))
    }
}
