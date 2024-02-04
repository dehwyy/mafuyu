use sea_orm::{prelude::*, IntoActiveValue};
use makoto_db::{models::user_languages, repo::languages::get_language_id_by_name};
use makoto_lib::errors::prelude::{RepositoryError, HandleError};

pub struct LanguagesRepo {
    db: DatabaseConnection
}

impl LanguagesRepo {
    pub fn new(db: DatabaseConnection) -> Self {
        Self {
            db
        }
    }

    pub async fn set_languages(&self, user_id: &Uuid,languages: Vec<String>) -> Result<(), RepositoryError> {
        let models: Vec<user_languages::ActiveModel> = languages.iter().filter_map(|language| {
            get_language_id_by_name(language).map(|id| {
              user_languages::ActiveModel {
                  user_id: user_id.clone().into_active_value(),
                  language_id: (id as i64).into_active_value(),
              }
            })

        }).collect();

        user_languages::Entity::insert_many(models).exec(&self.db).await.handle().map(|_| {})
    }
}