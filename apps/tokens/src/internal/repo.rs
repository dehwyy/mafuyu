use sea_orm::{ActiveValue, DatabaseConnection, IntoActiveValue};
use sea_orm::prelude::*;
use makoto_db::models::prelude::UserTokens;
use makoto_db::models::user_tokens;

use makoto_lib::errors::prelude::*;

pub enum GetTokenRecordBy {
    UserId(Uuid),
    AccessToken(String),
    RefreshToken(String)
}

#[derive(Clone)]
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
        let filter = match get_by {
            GetTokenRecordBy::UserId(id) => user_tokens::Column::UserId.eq(id),
            // I was struggling in about 2 hour to make this query...
            GetTokenRecordBy::AccessToken(token) => Expr::cust(format!(
                r#"'{token}' = ANY("{user_tokens}"."access_token")"#,
                token=token,
                user_tokens=user_tokens::Column::AccessToken.entity_name().to_string(),
            )),
            GetTokenRecordBy::RefreshToken(token) => user_tokens::Column::RefreshToken.eq(token)
        };

        let token_record = UserTokens::find()
            .filter(filter)
            .one(&self.db)
            .await.handle()?.safe_unwrap("token not found")?;

        Ok(token_record)
    }

    /// * Creates || updates `user_tokens::Model`
    pub async fn insert_tokens(&self, user_id: Uuid, access_token: &str, refresh_token: &str) -> Result<(), RepositoryError> {
        let tokens_model = self.get_token_record(GetTokenRecordBy::UserId(user_id.clone())).await;

        let active_model = match tokens_model {
            Ok(v) => {
                let mut tokens_active_model: user_tokens::ActiveModel = v.into();

                let mut tokens = tokens_active_model.access_token.take().unwrap_or_default().unwrap_or_default();
                tokens.push(access_token.to_string());
                tokens_active_model.refresh_token = Some(refresh_token.to_string()).into_active_value();

                Ok(tokens_active_model)
            },
            Err(err) => {
                match err {
                    RepositoryError::DbError(err) => Err(RepositoryError::DbError(err)),
                    RepositoryError::NotFound(_) => {
                        Ok(user_tokens::ActiveModel {
                            user_id: user_id.into_active_value(),
                            refresh_token: Some(refresh_token.to_string()).into_active_value(),
                            access_token: ActiveValue::Set(Some(vec!(access_token.to_string()))),
                            provider: "native".to_string().into_active_value(),
                            ..Default::default()
                        })
                    }
                }
            }
        }?;




        active_model.update(&self.db).await.handle()?;

        Ok(())
    }

    pub async fn set_access_tokens(&self, model: user_tokens::Model, new_access_token: Vec<String>) -> Result<(), RepositoryError> {
        let mut tokens_active_model: user_tokens::ActiveModel = model.into();

        tokens_active_model.access_token = ActiveValue::Set(Some(new_access_token));

        tokens_active_model.update(&self.db).await.handle()?;

        Ok(())
    }

}