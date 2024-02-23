use sea_orm::{DatabaseConnection, prelude::*, IntoActiveValue};
use sea_orm::sea_query::OnConflict;

use makoto_lib::errors::RepositoryError;
use makoto_db::models::{user_blocked, prelude::UserBlocked};
use makoto_lib::errors::prelude::HandleError;

pub struct BlockedUsersRepo {
    db: DatabaseConnection
}

impl BlockedUsersRepo {
    pub fn new(db: DatabaseConnection) -> Self {
        Self {
            db
        }
    }

    pub async fn block(&self, user_id: &Uuid, user_id_to_block: &Uuid) -> Result<(), RepositoryError> {
        UserBlocked::insert(user_blocked::ActiveModel {
            user_id: user_id.into_active_value(),
            blocked_user_id: user_id_to_block.into_active_value()
        }).on_conflict(OnConflict::new().do_nothing().to_owned()).exec(&self.db).await.handle().map(|_| {})
    }

    pub async fn unblock(&self, user_id: &Uuid, user_id_to_unblock: &Uuid) -> Result<(), RepositoryError> {
        UserBlocked::delete(user_blocked::ActiveModel {
            user_id: user_id.into_active_value(),
            blocked_user_id: user_id_to_unblock.into_active_value()
        }).exec(&self.db).await.handle().map(|_| {})
    }
}

