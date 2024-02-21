use sea_orm::{prelude::*, IntoActiveValue, QuerySelect};

use makoto_db::{models::{languages, prelude::{UserLanguages, Languages}, user_languages}, repo::languages::get_language_id_by_name};
use makoto_lib::errors::{prelude::HandleError, RepositoryError};

pub struct LanguagesRepo {
    db: DatabaseConnection
}

impl LanguagesRepo {
    pub fn new(db: DatabaseConnection) -> Self {
        Self {
            db
        }
    }

    pub async fn get_languages(&self, user_id: &Uuid) -> Result<Vec<String>, RepositoryError> {
        let languages  = UserLanguages::find()
            .find_with_related::<Languages>(Languages).filter(user_languages::Column::UserUserId.eq(user_id.clone())).all(&self.db).await.handle()?;

        let languages: Vec<String> = languages.iter().map(|language_entity| {
            language_entity.1.iter().map(|language| language.clone().name).collect::<Vec<String>>()
        }).flatten().collect();

        Ok(languages)
    }

    pub async fn set_languages(&self, user_id: &Uuid,languages: Vec<String>) -> Result<(), RepositoryError> {

        // Remove previous entities
        UserLanguages::delete_many()
            .filter(user_languages::Column::UserUserId.eq(user_id.clone()))
            .exec(&self.db).await.handle()?;

        let models: Vec<user_languages::ActiveModel> = languages.iter().filter_map(|language| {
            get_language_id_by_name(language).map(|id| {
              user_languages::ActiveModel {
                  user_user_id: user_id.clone().into_active_value(),
                  language_id: (id as i64).into_active_value(),
              }
            })

        }).collect();

        if models.len() != 0 {
            user_languages::Entity::insert_many(models).exec(&self.db).await.handle()?;
        }

        Ok(())
    }
}
