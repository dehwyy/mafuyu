use sea_orm::{DatabaseConnection, prelude::*, IntoActiveValue};
use sea_orm::sea_query::OnConflict;

use mafuyu_lib::errors::prelude::*;
use makoto_db::models::{user_blocked, prelude::UserBlocked};

pub struct BlockedUsersRepo {
    db: DatabaseConnection
}

impl BlockedUsersRepo {
    pub fn new(db: DatabaseConnection) -> Self {
        Self {
            db
        }
    }

    pub async fn get_blocked_users(&self, user_id: &Uuid) -> Result<Vec<user_blocked::Model>, RepositoryError> {
        UserBlocked::find().filter(user_blocked::Column::UserId.eq(*user_id)).all(&self.db).await.handle()
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

