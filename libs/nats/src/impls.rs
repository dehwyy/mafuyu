use crate::message::MessageError;
use crate::route::RouteError;
use crate::tools::SubjectError;

impl std::fmt::Display for RouteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s=  match self {
            Self::MessageError(s) => s.to_string(),
            Self::RepoError(s) => s.to_string()
        };

        write!(f, "{}", s)
    }
}

impl std::fmt::Display for MessageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::CannotDeserialize(s) => s.to_string(),
            Self::CannotSerialize(s) => s.to_string(),
            Self::MalformedRequest(s) => s.to_string(),
            Self::Internal(s) => s.to_string()
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