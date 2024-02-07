use sea_orm::{DatabaseConnection, IntoActiveValue, prelude::*};
use uuid::Uuid;
use makoto_db::models::user_credentials;
use makoto_lib::errors::{prelude::HandleError, RepositoryError};

pub use makoto_db::repo::credentials::GetCredentialsRecordBy;

pub struct CreateUserPayload {
    pub username: String,
    pub email: Option<String>,
    pub password: Option<String>,
    pub provider_id: Option<String>
}

pub struct Repo {
    db: DatabaseConnection
}

impl Repo {
    pub fn new(db: DatabaseConnection) -> Self {
        Self {
            db
        }
    }

    pub async fn get_user(&self, get_by: GetCredentialsRecordBy) -> Result<user_credentials::Model, RepositoryError> {
        makoto_db::repo::credentials::get_user(&self.db, get_by).await
    }

    pub async fn create_user(&self, p: CreateUserPayload) -> Result<user_credentials::Model, RepositoryError> {
        let user = user_credentials::ActiveModel {
            id: Uuid::new_v4().into_active_value(),
            username: p.username.into_active_value(),
            email: p.email.into_active_value(),
            password: p.password.into_active_value(),
            provider_id: p.provider_id.into_active_value(),
            ..Default::default()
        };

        Ok(user.insert(&self.db).await.handle()?)
    }
}