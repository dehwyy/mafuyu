use tonic::Status;

pub trait HandleError<T, R> {
  fn handle(self) -> Result<T, R>;
}

pub trait HandleNatsError<T> {
  fn handle_nats(self) -> Result<T, mafuyu_nats::route::RouteError>;
}

pub trait SafeUnwrapWithMessage<T, R> {
  fn safe_unwrap(self, msg: &str) -> Result<T, R>;
}

pub trait GrpcHandleError<T> {
  fn internal_error(self) -> Result<T, Status>;
  fn invalid_argument_error(self) -> Result<T, Status>;
  fn not_found_error(self) -> Result<T, Status>;
  fn already_exists_error(self) -> Result<T, Status>;
  fn permission_denied_error(self) -> Result<T, Status>;
  fn unauthenticated_error(self) -> Result<T, Status>;
}

pub trait HandleRepositoryError<T> {
  fn is_not_found(&self) -> bool;
}

pub mod repository {
  #[derive(Debug)]
  pub enum RepositoryError {
    DbError(String),
    NotFound(String),
  }

  impl std::fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      let s = match self {
        Self::NotFound(v) => v,
        Self::DbError(v) => v
      };

      write!(f, "{}", s)
    }
  }
}

pub mod prelude {
  pub use super::*;
  pub use repository::*;
}