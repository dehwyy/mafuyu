use super::traits::{HandleError, ResultedOption};
use sea_orm::{ModelTrait as SeaOrmModel, DbErr as SeaOrmDbErr};


#[derive(Clone, Debug)]
pub enum RepositoryError {
    DbError(String),
    NotFound(String),
    AlreadyExists(String),
    WrongPayload(String),
    InternalError(String)
}

impl std::fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::NotFound(v) => v,
            Self::AlreadyExists(v) => v,
            Self::DbError(v) => v,
            Self::WrongPayload(v) => v,
            Self::InternalError(v) => v
        };

        write!(f, "{}", s)
    }
}

impl<T> HandleError<T, RepositoryError> for Result<T, SeaOrmDbErr> {
    fn handle(self) -> Result<T, RepositoryError> {
        self.map_err(|err| RepositoryError::DbError(err.to_string()))
    }
}

#[cfg(feature = "rpc")]
impl<T> HandleError<T, tonic::Status> for Result<T, RepositoryError> {
    fn handle(self) -> Result<T, tonic::Status> {
        self.map_err(|err| {
            match err {
                RepositoryError::DbError(err) => tonic::Status::internal(err),
                RepositoryError::NotFound(err) => tonic::Status::not_found(err),
                RepositoryError::AlreadyExists(err) => tonic::Status::already_exists(err),
                RepositoryError::WrongPayload(err) => tonic::Status::invalid_argument(err),
                RepositoryError::InternalError(err) => tonic::Status::internal(err)
            }
        })
    }
}

impl<T> ResultedOption<T, RepositoryError> for Option<T>
    where T: SeaOrmModel
{
    fn unwrap_or_internal(self, msg: &str) -> Result<T, RepositoryError> {
        match self {
            Some(v) => Ok(v),
            None => Err(RepositoryError::InternalError(msg.to_string()))
        }
    }
    fn unwrap_or_not_found(self, msg: &str) -> Result<T, RepositoryError> {
        match self {
            Some(v) => Ok(v),
            None => Err(RepositoryError::NotFound(msg.to_string()))
        }
    }

    /// Shouldn't be called, I guess (only for tonic)
    fn unwrap_or_unauthorized(self, msg: &str) -> Result<T, RepositoryError> {
        match self {
            Some(v) => Ok(v),
            None => Err(RepositoryError::InternalError(msg.to_string()))
        }
    }
}