use sea_orm::{DatabaseConnection, EntityTrait, prelude::*};
use makoto_db::models::user_tokens;
use mafuyu_lib::errors::prelude::RepositoryError;

pub use makoto_db::repo::tokens::GetTokenRecordBy;

pub struct Tokens {
  db: DatabaseConnection,
}

impl Tokens {
  pub fn new(db: DatabaseConnection) -> Self {
    Self {
      db,
    }
  }


  pub async fn get_token_record(&self, get_by: GetTokenRecordBy) -> Result<user_tokens::Model, RepositoryError> {
    makoto_db::repo::tokens::get_token_record(&self.db, get_by).await
  }
}
