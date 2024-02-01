use sea_orm::{DatabaseConnection, IntoActiveValue, prelude::*};
use uuid::Uuid;
use makoto_db::models::user_credentials;
use makoto_lib::errors::prelude::{RepositoryError, HandleError};

pub struct CreateUserPayload {
    pub username: String,
    pub email: Option<String>,
    pub password: Option<String>
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

    pub async fn create_user(&self, user_payload: CreateUserPayload) -> Result<user_credentials::Model, RepositoryError> {
        let user = user_credentials::ActiveModel {
            id: Uuid::new_v4().into_active_value(),
            username: user_payload.username.into_active_value(),
            email: user_payload.email.into_active_value(),
            password: user_payload.password.into_active_value(),
            ..Default::default()
        };

        Ok(user.insert(&self.db).await.handle()?)
    }
}