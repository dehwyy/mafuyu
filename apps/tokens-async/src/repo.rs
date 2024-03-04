use sea_orm::{ActiveValue, DatabaseConnection};
use sea_orm::prelude::*;
use makoto_db::models::user_tokens;

use mafuyu_lib::errors::prelude::*;

pub use makoto_db::repo::tokens::GetTokenRecordBy;

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
        makoto_db::repo::tokens::get_token_record(&self.db, get_by).await
    }

    pub async fn set_access_tokens(&self, model: user_tokens::Model, new_access_token: Vec<String>) -> Result<(), RepositoryError> {
        let mut tokens_active_model: user_tokens::ActiveModel = model.into();

        tokens_active_model.access_tokens = ActiveValue::Set(Some(new_access_token));

        tokens_active_model.update(&self.db).await.handle()?;

        Ok(())
    }

}