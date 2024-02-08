use sea_orm::{IntoActiveValue, prelude::*, TryFromU64};
use makoto_lib::errors::{RepositoryError, prelude::HandleError};

use makoto_db::models::users as user;
use makoto_db::repo::user::get_user;

pub use makoto_db::repo::user::GetUserRecordBy;

pub struct EditPrimitiveUserPayload {
    pub user_id: Uuid,
    pub location: Option<String>,
    pub birthday: Option<u64>,
    pub pseudonym: Option<String>,
    pub bio: Option<String>,
    pub picture: Option<String>,
}

pub struct UserRepo {
    db: DatabaseConnection
}

impl UserRepo {
    pub fn new(db: DatabaseConnection) -> Self {
        Self {
            db
        }
    }

    pub async fn get_user(&self, get_by: GetUserRecordBy) -> Result<user::Model, RepositoryError> {
        get_user(&self.db, get_by).await
    }

    pub async fn create_basic_user(&self, user_id: Uuid, picture: Option<String>) -> Result<user::Model, RepositoryError> {
        let model = user::ActiveModel {
            user_id: user_id.into_active_value(),
            picture: picture.into_active_value(),
            ..Default::default()
        };

        model.insert(&self.db).await.handle()
    }

    pub async fn edit_primitive_user(&self, p: EditPrimitiveUserPayload) -> Result<user::Model, RepositoryError> {
        let mut user: user::ActiveModel = self.get_user(GetUserRecordBy::UserId(p.user_id)).await?.into();

        user.birthday = match p.birthday {
            Some(timestamp) => {
                ChronoDateTimeWithTimeZone::try_from_u64(timestamp).map(|t| Some(t)).handle()
            },
            None => Ok(None)
        }?.into_active_value();

        user.pseudonym = p.pseudonym.into_active_value();
        user.bio = p.bio.into_active_value();
        user.location = p.location.into_active_value();
        user.picture = p.picture.into_active_value();

        user.update(&self.db).await.handle()
    }
}
