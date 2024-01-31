use sea_orm::{ActiveValue, DatabaseConnection};
use sea_orm::prelude::*;
use makoto_db::models::prelude::UserTokens;
use makoto_db::models::user_tokens;

use makoto_lib::errors::*;
use makoto_lib::errors::repository::RepositoryError;
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
                user_tokens=user_tokens::Column::AccessTokens.entity_name().to_string(),
            )),
            GetTokenRecordBy::RefreshToken(token) => user_tokens::Column::RefreshToken.eq(token)
        };

        let token_record = UserTokens::find()
            .filter(filter)
            .one(&self.db)
            .await.handle()?.safe_unwrap("token not found")?;

        Ok(token_record)
    }

    pub async fn set_access_tokens(&self, model: user_tokens::Model, new_access_token: Vec<String>) -> Result<(), RepositoryError> {
        let mut tokens_active_model: user_tokens::ActiveModel = model.into();

        tokens_active_model.access_tokens = ActiveValue::Set(Some(new_access_token));

        tokens_active_model.update(&self.db).await.handle()?;

        Ok(())
    }

}