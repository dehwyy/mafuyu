use tonic::Status;
use mafuyu_nats::errors::{MessageError};
use super::traits::{HandleError, ResultedOption};


impl<T> HandleError<T, Status> for Result<T, MessageError> {
    fn handle(self) -> Result<T, Status> {
        self.map_err(|err| {
            match err {
                MessageError::MalformedRequest(err) => Status::invalid_argument(err),
                MessageError::CannotDeserialize(err) => Status::internal(err.to_string()),
                _ => Status::internal(err.to_string())
            }
        })
    }
}

impl<T> HandleError<T, Status> for Result<T, async_nats::PublishError> {
    fn handle(self) -> Result<T, Status> {
        self.map_err(|err| Status::internal(err.to_string()))
    }
}

impl<T> ResultedOption<T, Status> for Option<T> {
    fn unwrap_or_internal(self, msg: &str) -> Result<T, Status> {
        match self {
            Some(v) => Ok(v),
            None => Err(Status::internal(msg))
        }
    }
    fn unwrap_or_not_found(self, msg: &str) -> Result<T, Status> {
        match self {
            Some(v) => Ok(v),
            None => Err(Status::not_found(msg))
        }
    }

    fn unwrap_or_unauthorized(self, msg: &str) -> Result<T, Status> {
        match self {
            Some(v) => Ok(v),
            None => Err(Status::unauthenticated(msg))
        }
    }
}