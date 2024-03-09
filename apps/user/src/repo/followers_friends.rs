use sea_orm::{DatabaseConnection, prelude::*, IntoActiveValue, TransactionTrait, IntoActiveModel, Condition, QuerySelect};
use sea_orm::sea_query::OnConflict;
use uuid::Uuid;
use mafuyu_lib::errors::prelude::*;
use makoto_db::models::{user_followers, user_friends, prelude::{UserFollowers, UserFriends}};

pub struct FollowersFriendsRepo {
    db: DatabaseConnection
}

impl FollowersFriendsRepo {
    pub fn new(db: DatabaseConnection) -> Self {
        Self {
            db
        }
    }

    pub async fn get_followers(&self, user_id: &Uuid, limit: Option<u32>) -> Result<Vec<user_followers::Model>, RepositoryError> {
        UserFollowers::find()
            .filter(user_followers::Column::FollowedToUserId.eq(*user_id))
            .limit(limit.map(|v| v as u64))
            .all(&self.db).await.handle()
    }

    pub async fn get_followed_to(&self, user_id: &Uuid, limit: Option<u32>) -> Result<Vec<user_followers::Model>, RepositoryError> {
        UserFollowers::find()
            .filter(user_followers::Column::UserId.eq(*user_id))
            .limit(limit.map(|v| v as u64))
            .all(&self.db).await.handle()
    }

    pub async fn get_friends(&self, user_id: &Uuid, limit: Option<u32>) -> Result<Vec<user_friends::Model>, RepositoryError> {
        UserFriends::find()
            .filter(user_friends::Column::UserId.eq(*user_id))
            .limit(limit.map(|v| v as u64))
            .all(&self.db).await.handle()
    }

    pub async fn follow(&self, user_id: &Uuid, follow_to_user_id: &Uuid) -> Result<(), RepositoryError> {
        match self.get_followers_relation(follow_to_user_id, user_id).await?
        {
            //  both user will be followed to each other -> become friends
            Some(model) => {
                let txn = self.db.begin().await.handle()?;

                let model1 = user_friends::ActiveModel {
                    user_id: user_id.into_active_value(),
                    friend_user_id: follow_to_user_id.into_active_value(),
                };
                let model2 = user_friends::ActiveModel {
                    user_id: follow_to_user_id.into_active_value(),
                    friend_user_id: user_id.into_active_value(),
                };

                let friends_insert_future = UserFriends::insert_many([model1, model2]).exec(&self.db);
                UserFollowers::delete(model.into_active_model()).exec(&self.db).await.handle()?;

                friends_insert_future.await.handle()?;
                txn.commit().await.handle()?;
            },
            //  1st user will be followed to 2nd user
            None => {
                let model = user_followers::ActiveModel {
                    user_id: user_id.into_active_value(),
                    followed_to_user_id: follow_to_user_id.into_active_value(),
                };

                UserFollowers::insert(model)
                    .on_conflict(OnConflict::new().do_nothing().to_owned()).exec(&self.db).await.handle()?;
            }
        };

        Ok(())
    }

    pub async fn unfollow(&self, user_id: &Uuid, unfollow_user_id: &Uuid) -> Result<(), RepositoryError> {
        match self.get_friends_relation(user_id, unfollow_user_id).await? {
            // Users were friends -> remove 2 relations and make 2nd user followed
            Some(_) => {
                let txn = self.db.begin().await.handle()?;

                // remove 2 records
                let friends_delete_future = UserFriends::delete_many()
                    .filter(
                        Condition::any()
                            .add(Condition::all().add(user_friends::Column::UserId.eq(*user_id))         .add(user_friends::Column::FriendUserId.eq(*unfollow_user_id)))
                            .add(Condition::all().add(user_friends::Column::UserId.eq(*unfollow_user_id)).add(user_friends::Column::FriendUserId.eq(*user_id)))
                    )
                    .exec(&self.db);
                // make 2nd user followed
                UserFollowers::insert(user_followers::ActiveModel {
                    user_id: unfollow_user_id.into_active_value(),
                    followed_to_user_id: user_id.into_active_value(),
                }).exec(&self.db).await.handle()?;

                friends_delete_future.await.handle()?;
                txn.commit().await.handle()?;
            },
            // Users were not friends -> just unfollow
            None => UserFollowers::delete(user_followers::ActiveModel {
                user_id: user_id.into_active_value(),
                followed_to_user_id: unfollow_user_id.into_active_value()
            }).exec(&self.db).await.map(|_| {}).handle()?
        };

        Ok(())
    }

    async fn get_followers_relation(&self, follower_user_id: &Uuid, followed_to_user_id: &Uuid) -> Result<Option<user_followers::Model>, RepositoryError> {
        Ok(
            UserFollowers::find()
                .filter(
                    Condition::all()
                        .add(user_followers::Column::UserId.eq(*follower_user_id))
                        .add(user_followers::Column::FollowedToUserId.eq(*followed_to_user_id))
                ).one(&self.db).await.handle()?
        )
    }
    async fn get_friends_relation(&self, user_id: &Uuid, friend_id: &Uuid) -> Result<Option<user_friends::Model>, RepositoryError> {
        Ok(
            UserFriends::find()
                .filter(
                    Condition::all()
                        .add(user_friends::Column::UserId.eq(*user_id))
                        .add(user_friends::Column::FriendUserId.eq(*friend_id))
                ).one(&self.db).await.handle()?
        )
    }
}
