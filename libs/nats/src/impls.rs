use crate::errors::*;

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