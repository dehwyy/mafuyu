mod traits;
mod impls;

pub use traits::NatsHandleError;

#[derive(Debug)]
pub enum HandleError {
    CannotAck(String),
    SubjectError(SubjectError),
    SubjectNotFound(String),
    RouteError(RouteError),
}

#[derive(Debug)]
pub enum SubjectError {
    MalformedSubjectName(String)
}

pub enum MessageError {
    CannotSerialize(String),
    CannotDeserialize(String),
    MalformedRequest(String),
    Internal(String),
}

#[derive(Debug)]
pub enum RouteError {
    InvalidArgument(String),
    NotFound(String),
    AlreadyExists(String),
    InternalError(String),
}