use sea_orm::{DatabaseConnection, IntoActiveValue, prelude::*};
use uuid::Uuid;
use makoto_db::models::user_credentials;
use mafuyu_lib::errors::prelude::*;
use makoto_db::models::prelude::UserCredentials;

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

    pub async fn update_username(&self, user_id: Uuid, username: String) -> Result<user_credentials::Model, RepositoryError> {
        user_credentials::ActiveModel {
            id: user_id.into_active_value(),
            username: username.into_active_value(),
            ..Default::default()
        }.update(&self.db).await.handle()
    }

    pub async fn create_user(&self, p: CreateUserPayload) -> Result<Uuid, RepositoryError> {
        let r = UserCredentials::insert(user_credentials::ActiveModel {
            id: Uuid::new_v4().into_active_value(),
            username: p.username.into_active_value(),
            email: p.email.into_active_value(),
            password: p.password.into_active_value(),
            provider_id: p.provider_id.into_active_value(),
            ..Default::default()
        }).exec(&self.db).await.handle()?;

        Ok(r.last_insert_id)
    }
}
