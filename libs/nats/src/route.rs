
pub enum RouteError {
    MessageError(crate::message::MessageError),
    RepoError(String)
}

pub type RouteResult = Result<(), RouteError>;