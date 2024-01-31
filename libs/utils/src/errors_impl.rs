use tonic::Status;
use mafuyu_nats::route::RouteError;
use crate::errors::{GrpcHandleError, HandleError, HandleNatsError, HandleRepositoryError, SafeUnwrapWithMessage};
use crate::errors::repository::RepositoryError;

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
impl<T> HandleNatsError<T> for Result<T, RepositoryError> {
    fn handle_nats(self) -> Result<T, RouteError> {
        match self {
            Ok(v) => Ok(v),
            Err(err) => {
                Err(RouteError::RepoError(err.to_string()))
            }
        }
    }
}


impl<T> HandleError<T, RouteError> for Result<T, RepositoryError> {
    fn handle(self) -> Result<T, RouteError> {
        todo!()
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
impl<T> HandleError<T, RepositoryError> for Result<T, sea_orm::DbErr> {
    /// `Default` database error handler
    fn handle(self) -> Result<T, RepositoryError> {
        self.map_err(|err| RepositoryError::DbError(err.to_string()))
    }
}

impl<T> SafeUnwrapWithMessage<T, Status> for Option<T> {
    fn safe_unwrap(self, msg: &str) -> Result<T, Status> {
        match self {
            Some(v) => Ok(v),
            None => Err(Status::internal(msg))
        }
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
