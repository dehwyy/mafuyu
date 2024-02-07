pub enum SubjectError {
    MalformedSubjectName(String)
}

pub enum MessageError {
    CannotSerialize(String),
    CannotDeserialize(String),
    MalformedRequest(String),
    Internal(String),
}

pub enum RouteError {
    InvalidArgument(String),
    NotFound(String),
    AlreadyExists(String),
    InternalError(String),
}

pub trait NatsHandleError<T> {
    fn internal_error(self) -> Result<T, RouteError>;
    fn invalid_argument_error(self) -> Result<T, RouteError>;
    fn not_found_error(self) -> Result<T, RouteError>;
    fn already_exists_error(self) -> Result<T, RouteError>;
}

impl<T, R> NatsHandleError<T> for Result<T, R>
    where R: std::fmt::Display
{
    fn internal_error(self) -> Result<T, RouteError> {
        self.map_err(|err| RouteError::InternalError(err.to_string()))
    }
    fn invalid_argument_error(self) -> Result<T, RouteError> {
        self.map_err(|err| RouteError::InvalidArgument(err.to_string()))
    }
    fn not_found_error(self) -> Result<T, RouteError> {
        self.map_err(|err| RouteError::NotFound(err.to_string()))
    }
    fn already_exists_error(self) -> Result<T, RouteError> {
        self.map_err(|err| RouteError::AlreadyExists(err.to_string()))
    }
}