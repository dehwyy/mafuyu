use makoto_lib::errors::prelude::*;

pub mod user {
    use sea_orm::prelude::*;
    use sea_orm::sea_query::SimpleExpr;
    use makoto_lib::errors::{HandleError, SafeUnwrapWithMessage};
    use makoto_lib::errors::prelude::RepositoryError;
    use crate::models::prelude::{Users as User, UserCredentials};
    use crate::models::{users as user, user_credentials};

    pub enum GetUserRecordBy {
        UserId(Uuid),
        Username(String)
    }

    pub async fn get_user(db_conn: &DatabaseConnection, get_by: GetUserRecordBy) -> Result<user::Model, RepositoryError> {

        match get_by {
            GetUserRecordBy::Username(username) => {
                let joined_model =  User::find()
                    .find_also_related(UserCredentials).filter(user_credentials::Column::Username.eq(username))
                    .one(db_conn).await.handle()?;

                let model = match joined_model {
                    Some(v) => Ok(v),
                    None => Err(RepositoryError::NotFound("user with user_credentials wasn't found".to_string()))
                }?.0; // 1st element of the tuple

                Ok(model)
            },
            get_by => {
                let filter: Option<SimpleExpr> = match get_by {
                    GetUserRecordBy::UserId(user_id) => Some(user::Column::UserId.eq(user_id)),
                    _ => None
                };

                match filter {
                    Some(filter) => {
                        let user = User::find().filter(filter).one(db_conn).await.handle()?;

                        Ok(user.safe_unwrap("user(info) not found")?)
                    },
                    None => Err(RepositoryError::NotFound("invalid filter".to_string()))
                }
            }
        }
    }
}

pub mod credentials {
    use super::*;
    use sea_orm::prelude::*;
    use makoto_lib::errors::prelude::RepositoryError;
    use crate::models::prelude::UserCredentials;
    use crate::models::user_credentials;

    pub enum GetCredentialsRecordBy {
        UserId(Uuid),
        Username(String),
        Email(String),
        ProviderId(String)
    }

    pub async fn get_user(db_connection: &DatabaseConnection, get_by: GetCredentialsRecordBy) -> Result<user_credentials::Model, RepositoryError> {
        let filter = match get_by {
            GetCredentialsRecordBy::UserId(user_id) => user_credentials::Column::Id.eq(user_id),
            GetCredentialsRecordBy::Username(username) => user_credentials::Column::Username.eq(username),
            GetCredentialsRecordBy::Email(email) => user_credentials::Column::Email.eq(Some(email)),
            GetCredentialsRecordBy::ProviderId(provider_id) => user_credentials::Column::ProviderId.eq(Some(provider_id))
        };

        let user = UserCredentials::find().filter(filter).one(db_connection).await.handle()?;

        Ok(user.safe_unwrap("user wasn't found")?)
    }
}

pub mod tokens {
    use super::*;
    use sea_orm::prelude::*;
    use crate::models::prelude::UserTokens;
    use crate::models::user_tokens;

    pub enum GetTokenRecordBy {
        UserId(Uuid),
        AccessToken(String),
        RefreshToken(String)
    }

    pub async fn get_token_record(db_connection: &DatabaseConnection, get_by: GetTokenRecordBy) -> Result<user_tokens::Model, RepositoryError> {
        let filter = match get_by {
            GetTokenRecordBy::UserId(id) => user_tokens::Column::UserId.eq(id),
            // I was struggling in about 2 hour to make this query...
            GetTokenRecordBy::AccessToken(token) => Expr::cust(format!(
                r#"'{token}' = ANY("{user_tokens}"."access_tokens")"#,
                token=token,
                user_tokens=user_tokens::Column::AccessTokens.entity_name().to_string(),
            )),
            GetTokenRecordBy::RefreshToken(token) => user_tokens::Column::RefreshToken.eq(token)
        };

        let token_record = UserTokens::find()
            .filter(filter)
            .one(db_connection)
            .await.handle()?;

        Ok(token_record.safe_unwrap("token not found")?)
    }
}

pub mod languages {
    static LANGUAGES: phf::Map<&'static str, u32> = phf::phf_map! {
        "arabic" => 1,
        "dutch" => 2,
        "english" => 3,
        "french" => 4,
        "german" => 5,
        "hindi" => 6,
        "indonesian" => 7,
        "italian" => 8,
        "japanese" => 9,
        "korean" => 10,
        "chinese" => 11,
        "polish" => 12,
        "portuguese" => 13,
        "russian" => 14,
        "spanish" => 15,
        "thai" => 16,
        "turkish" => 17,
    };

    pub fn get_language_id_by_name(name: &str) -> Option<u32> {
        LANGUAGES.get(name).map(|v| v.clone())
    }
}