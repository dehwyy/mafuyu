use makoto_lib::errors::prelude::*;

pub mod credentials {
    use super::*;
    use sea_orm::prelude::*;
    use makoto_lib::errors::prelude::RepositoryError;
    use crate::models::prelude::UserCredentials;
    use crate::models::user_credentials;

    pub enum GetRecordBy {
        UserId(Uuid),
        Username(String),
        Email(String)
    }

    pub async fn get_user(db_connection: &DatabaseConnection, get_by: GetRecordBy) -> Result<user_credentials::Model, RepositoryError> {
        let filter = match get_by {
            GetRecordBy::UserId(user_id) => user_credentials::Column::Id.eq(user_id),
            GetRecordBy::Username(username) => user_credentials::Column::Username.eq(username),
            GetRecordBy::Email(email) => user_credentials::Column::Email.eq(email)
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
                r#"'{token}' = ANY("{user_tokens}"."access_token")"#,
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