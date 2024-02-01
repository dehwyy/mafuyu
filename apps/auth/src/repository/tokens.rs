use sea_orm::{DatabaseConnection, EntityTrait, prelude::*};
use makoto_db::models::user_tokens;
use makoto_lib::errors::prelude::*;

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
