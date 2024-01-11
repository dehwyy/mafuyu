

pub mod repository {
  use sea_orm::DbErr as SeaOrmDbError;

  pub enum RepositoryError {
    DbError(String),
    NotFound(String),
  }

  impl RepositoryError {
    pub fn to_string(&self) -> String {
      match self {
        RepositoryError::DbError(err) => err.clone(),
        RepositoryError::NotFound(err) => err.clone(),
      }
    }
  }

  pub mod prelude {
  use super::*;

  pub trait HandleDbError<T> {
    fn handle(self) -> Result<T, RepositoryError>;
  }

  impl<T> HandleDbError<T> for Result<T, SeaOrmDbError> {
    /// `Default` database error handler
    fn handle(self) -> Result<T, RepositoryError> {
      self.map_err(|err| RepositoryError::DbError(err.to_string()))
    }
  }

  pub trait NotFoundModel<T>
  where T: sea_orm::ModelTrait {
    fn extract(self, message_on_error: &str) -> Result<T, RepositoryError>;
  }

  impl<T> NotFoundModel<T> for Option<T>
  where T: sea_orm::ModelTrait {
    /// `Default` behavior for handling `NotFoundRecord` cases
    fn extract(self, message_on_error: &str) -> Result<T, RepositoryError> {
      match self {
        Some(model) => Ok(model),
        None => Err(RepositoryError::NotFound(message_on_error.to_string()))
      }
    }
  }
}
}

pub mod service {
  use tonic::Status;

  pub mod prelude {
    use crate::errors::repository::RepositoryError;

    use super::*;

    pub trait HandleError<T> {
      fn internal_error(self) -> Result<T, Status>;
      fn invalid_argument_error(self) -> Result<T, Status>;
      fn not_found_error(self) -> Result<T, Status>;
      fn already_exists_error(self) -> Result<T, Status>;
      fn permission_denied_error(self) -> Result<T, Status>;
      fn unauthenticated_error(self) -> Result<T, Status>;
    }

    impl<T> HandleError<T> for Result<T, String> {
      fn internal_error(self) -> Result<T, Status> {
        self.map_err(|err| Status::internal(err))
      }
      fn invalid_argument_error(self) -> Result<T, Status> {
        self.map_err(|err| Status::invalid_argument(err))
      }
      fn not_found_error(self) -> Result<T, Status> {
        self.map_err(|err| Status::not_found(err))
      }
      fn already_exists_error(self) -> Result<T, Status> {
        self.map_err(|err| Status::already_exists(err))
      }
      fn permission_denied_error(self) -> Result<T, Status> {
        self.map_err(|err| Status::permission_denied(err))
      }
      fn unauthenticated_error(self) -> Result<T, Status> {
        self.map_err(|err| Status::unauthenticated(err))
      }
    }

    pub trait HandleRepositoryError<T> {
      fn handle(self) -> Result<T, Status>;
    }

    // Extracts `Value` from `Result` or returns boilplate `tonic::Status` based on kind of `RepositoryError`
    impl<T> HandleRepositoryError<T> for Result<T, RepositoryError> {
      fn handle(self) -> Result<T, Status> {
        match self {
          Ok(model) => Ok(model),
          Err(err) => {
            match err {
              RepositoryError::DbError(err) => Err(Status::internal(err)),
              RepositoryError::NotFound(err) => Err(Status::not_found(err)),
              _ => Err(Status::internal(err.to_string()))
            }
          }
        }
      }
    }
  }
}
