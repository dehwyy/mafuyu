use sea_orm::{ActiveValue, DatabaseConnection, IntoActiveValue};
use sea_orm::prelude::*;
use makoto_db::models::user_tokens;
use makoto_db::repo::tokens::get_token_record;
use makoto_lib::errors::prelude::*;

pub use makoto_db::repo::tokens::GetTokenRecordBy;

pub struct Repo {
    db: DatabaseConnection
}

impl Repo {
    pub fn new(db: DatabaseConnection) -> Self  {
        Self {
            db
        }
    }

    pub async fn get_token_record(&self, get_by: GetTokenRecordBy) -> Result<user_tokens::Model, RepositoryError> {
        get_token_record(&self.db, get_by).await
    }

    /// * Creates || updates `user_tokens::Model`
    pub async fn insert_tokens(&self, user_id: Uuid, access_token: String, refresh_token: Option<String>, provider: Option<String>) -> Result<(), RepositoryError> {
        let tokens_model = self.get_token_record(GetTokenRecordBy::UserId(user_id.clone())).await;

        let db_update = match tokens_model {
            Ok(v) => {
                let mut tokens_active_model: user_tokens::ActiveModel = v.into();

                let mut tokens = tokens_active_model.access_tokens.take().unwrap_or_default().unwrap_or_default();
                tokens.push(access_token);

                if let Some(refresh_token) = refresh_token {
                    tokens_active_model.refresh_token = Some(refresh_token).into_active_value();
                }

                tokens_active_model.access_tokens = ActiveValue::Set(Some(tokens));

                Ok(tokens_active_model.update(&self.db))
            },
            Err(err) => {
                match err {
                    RepositoryError::DbError(err) => Err(RepositoryError::DbError(err)),
                    RepositoryError::NotFound(_) => {
                        let model = user_tokens::ActiveModel {
                            user_id: user_id.into_active_value(),
                            refresh_token: refresh_token.into_active_value(),
                            access_tokens: ActiveValue::Set(Some(vec!(access_token))),
                            provider: ActiveValue::Set(provider),
                            ..Default::default()
                        };

                        Ok(model.insert(&self.db))
                    }
                }
            }
        }?;

       db_update.await.handle()?;

        Ok(())
    }
}