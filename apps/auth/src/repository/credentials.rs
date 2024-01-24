use sea_orm::{DatabaseConnection, prelude::Uuid, ColumnTrait, QueryFilter, EntityTrait, ActiveModelTrait};

use makoto_db::models::user_credentials::{Entity as UserCredentials, self};
use makoto_db::utilities::*;

use makoto_lib::errors::repository::{RepositoryError, prelude::*};

pub enum GetRecordBy {
  UserId(Uuid),
  Username(String),
  Email(String)
}

#[derive(Default)]
pub struct UserPayload {
  pub user_id: Uuid,
  pub username: String,
  pub email: Option<String>,
  pub password: Option<String>
}

pub struct Credentials {
  db: DatabaseConnection
}

impl Credentials {
  pub fn new(db: DatabaseConnection) -> Self {
    Self {
      db
    }
  }

  pub async fn create_user(&self, user_payload: UserPayload) -> Result<user_credentials::Model, RepositoryError> {
    let user = user_credentials::ActiveModel {
      id: not_null(user_payload.user_id),
      username: not_null(user_payload.username),
      email: not_null(user_payload.email),
      password: not_null(user_payload.password),
      ..Default::default()
    };

    Ok(user.insert(&self.db).await.handle()?)
  }

  pub async fn get_user(&self, get_by: GetRecordBy) -> Result<user_credentials::Model, RepositoryError> {
    let filter = match get_by {
      GetRecordBy::UserId(user_id) => user_credentials::Column::Id.eq(user_id),
      GetRecordBy::Username(username) => user_credentials::Column::Username.eq(username),
      GetRecordBy::Email(email) => user_credentials::Column::Email.eq(email)
    };

    let user = UserCredentials::find().filter(filter).one(&self.db).await.handle()?;

    user.extract("user not found")
  }
}
