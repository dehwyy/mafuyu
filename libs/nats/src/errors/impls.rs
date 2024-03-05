use crate::errors::*;

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

impl std::fmt::Display for RouteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s=  match self {
            Self::InternalError(s) => format!("[internal error]: {}", s),
            Self::AlreadyExists(s) => format!("[already exists]: {}", s),
            Self::NotFound(s) => format!("[not found]: {}", s),
            Self::InvalidArgument(s) => format!("[invalid argument]: {}", s),
        };

        write!(f, "{}", s)
    }
}

impl std::fmt::Display for MessageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::CannotDeserialize(s) => format!("[cannot deserialize]: {}", s),
            Self::CannotSerialize(s) => format!("[cannot serialize]: {}", s),
            Self::MalformedRequest(s) => format!("[malformed request]: {}", s),
            Self::Internal(s) => format!("[internal error]: {}", s)
        };
        write!(f, "{}", s)
    }
}

impl std::fmt::Display for SubjectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            SubjectError::MalformedSubjectName(s) => s.to_string()
        };
        write!(f, "{}", s)
    }
}