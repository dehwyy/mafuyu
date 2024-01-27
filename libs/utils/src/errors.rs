use tonic::Status;
use crate::errors::prelude::RepositoryError;

/// ### HandleError Trait
pub trait HandleError<T, R> {
  fn handle(self) -> Result<T, R>;
}

impl<T> HandleError<T, Status> for Result<T, RepositoryError> {
  fn handle(self) -> Result<T, Status> {
    match self {
      Ok(model) => Ok(model),
      Err(err) => {
        match err {
          RepositoryError::DbError(err) => Err(Status::internal(err)),
          RepositoryError::NotFound(err) => Err(Status::not_found(err))
        }
      }
    }
  }
}

impl<T> HandleError<T, Status> for Result<T, async_nats::RequestError> {
  fn handle(self) -> Result<T, Status> {
    match self {
      Ok(v) => Ok(v),
      Err(err) => Err(Status::internal(err.to_string()))
    }
  }
}

impl<T> HandleError<T, Status> for serde_json::Result<T> {
  fn handle(self) -> Result<T, Status> {
    match self {
      Ok(v) => Ok(v),
      Err(err) =>Err(Status::internal(err.to_string()))
    }
  }
}


/// ### SafeUnwrap Trait
pub trait SafeUnwrapWithMessage<T, R> {
  fn safe_unwrap(self, msg: &str) -> Result<T, R>;
}

impl<T> SafeUnwrapWithMessage<T, Status> for Option<T> {
  fn safe_unwrap(self, msg: &str) -> Result<T, Status> {
    match self {
      Some(v) => Ok(v),
      None => Err(Status::internal(msg))
    }
  }
}

/// ### GrpcHandleError Trait
pub trait GrpcHandleError<T> {
  fn internal_error(self) -> Result<T, Status>;
  fn invalid_argument_error(self) -> Result<T, Status>;
  fn not_found_error(self) -> Result<T, Status>;
  fn already_exists_error(self) -> Result<T, Status>;
  fn permission_denied_error(self) -> Result<T, Status>;
  fn unauthenticated_error(self) -> Result<T, Status>;
}

impl<T> GrpcHandleError<T> for Result<T, String> {
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

/// ### HandleRepositoryError Trait
// * Extracts `Value` from `Result` or returns boilerplate `tonic::Status` based on kind of `RepositoryError`
pub trait HandleRepositoryError<T> {
  fn is_not_found(&self) -> bool;
}

impl<T> HandleRepositoryError<T> for Result<T, RepositoryError> {
  fn is_not_found(&self) -> bool {
    if let Err(err) = &self {
      if let RepositoryError::NotFound(err) = err {
        return true;
      }
    }
    false
  }
}

mod repository {
  use std::fmt::Formatter;
  use super::*;

  #[derive(Debug)]
  pub enum RepositoryError {
    DbError(String),
    NotFound(String),
  }

  impl std::fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
      let s = match self {
        Self::NotFound(v) => v,
        Self::DbError(v) => v
      };

      write!(f, "{}", s)
    }
  }

  pub mod prelude {
    use crate::errors::HandleError;
    use super::*;

    impl<T> HandleError<T, RepositoryError> for Result<T, sea_orm::DbErr> {
      /// `Default` database error handler
      fn handle(self) -> Result<T, RepositoryError> {
        self.map_err(|err| RepositoryError::DbError(err.to_string()))
      }
    }

    impl<T> SafeUnwrapWithMessage<T, RepositoryError> for Option<T>
    where T: sea_orm::ModelTrait {
      fn safe_unwrap(self, msg: &str) -> Result<T, RepositoryError> {
        match self {
          Some(model) => Ok(model),
          None => Err(RepositoryError::NotFound(msg.to_string()))
        }
      }
    }

  }
}

pub mod prelude {
  pub use super::{GrpcHandleError, HandleRepositoryError, HandleError, SafeUnwrapWithMessage};
  pub use super::repository::RepositoryError;
}
