use async_nats::jetstream::Message;
use uuid::Uuid;
use makoto_lib::errors::prelude::*;
use makoto_logger::error;
use crate::internal::jwt::Jwt;
use crate::internal::repo::{GetTokenRecordBy, Repo as TokenRepo};
use crate::internal::router::payload::*;
use crate::internal::router::RouterError;

type RouteReturn = Result<(), RouterError>;

pub struct Router {
    repo: TokenRepo
}

impl Router {
    pub fn new(token_repo: TokenRepo) -> Self {
        Self {
            repo: token_repo
        }
    }

    pub async fn handle(&self, subject: &str, message: Message) {
        if let Err(err) =  message.ack().await {
            error!("[cannot ack] {err}");
            return;
        }

        let _r = match subject {
            "clear_tokens" => self.clear_tokens(message).await,
            _ => Err(
                RouterError::Internal(
                    format!("[unknown subject] {subject}"))
            )
        };

    }

    /// ### Removes `expired` `native` access_tokens from db
    /// * payload ( `JSON { user_id: String (Uuid), access_token: Option<String> } ` ), If provided => this token would removed to;
    async fn clear_tokens(&self, message: Message) ->  RouteReturn {

        let payload = get_payload::<ClearTokensPayload>(&message.payload)?;

        let user_id = Uuid::try_parse(&payload.user_id).map_err(|err| {
            RouterError::WrongPayload(err.to_string())
        })?;

        let record = self.repo.get_token_record(GetTokenRecordBy::UserId(user_id)).await.map_err(|err| {
            match err {
                RepositoryError::DbError(err) => RouterError::Internal(err),
                RepositoryError:: NotFound(err) => RouterError::Local(err)
            }
        })?;

        let validation_fn = match record.provider.as_str() {
            "native" => Ok(ValidationFunctions::native),
            _ => Err(RouterError::Internal("provider validation fn wasn't found".to_string()))
        }?;

        let access_tokens = record.access_token.clone().unwrap_or_default();
        let mut valid_access_tokens: Vec<String> = vec!();
        for token in access_tokens {
            if validation_fn(&token).await {
                valid_access_tokens.push(token)
            }
        }

        self.repo.set_access_tokens(record, valid_access_tokens).await.map_err(|err| {
            RouterError::Internal(err.to_string())
        })?;

        Ok(())
    }
}

struct ValidationFunctions;
impl ValidationFunctions {
    async fn native(access_token: &str) -> bool {
        Jwt::validate_access_token(access_token.to_string()).is_ok()
    }
}