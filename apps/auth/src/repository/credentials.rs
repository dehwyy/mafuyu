use sea_orm::{DatabaseConnection, IntoActiveValue};
use sea_orm::prelude::*;
use makoto_db::models::user_credentials;
use makoto_lib::errors::prelude::*;

pub use makoto_db::repo::credentials::GetRecordBy;

pub struct Credentials {
  db: DatabaseConnection
}

impl Credentials {
  pub fn new(db: DatabaseConnection) -> Self {
    Self {
      db
    }
  }

  pub async fn get_user(&self, get_by: GetRecordBy) -> Result<user_credentials::Model, RepositoryError> {
    makoto_db::repo::credentials::get_user(&self.db, get_by).await
  }
}
