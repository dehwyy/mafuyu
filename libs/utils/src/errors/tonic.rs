use tonic::Status;
use crate::errors::traits::ResultedOption;

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