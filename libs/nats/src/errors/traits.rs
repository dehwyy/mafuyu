use super::*;

pub trait NatsHandleError<T> {
    fn internal_error(self) -> Result<T, RouteError>;
    fn invalid_argument_error(self) -> Result<T, RouteError>;
    fn not_found_error(self) -> Result<T, RouteError>;
    fn already_exists_error(self) -> Result<T, RouteError>;
}