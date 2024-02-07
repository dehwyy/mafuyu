use std::fmt::Display;
use tonic::Status;

pub trait GrpcHandleError<T> {
    fn internal_error(self) -> Result<T, Status>;
    fn invalid_argument_error(self) -> Result<T, Status>;
    fn not_found_error(self) -> Result<T, Status>;
    fn already_exists_error(self) -> Result<T, Status>;
    fn permission_denied_error(self) -> Result<T, Status>;
    fn unauthenticated_error(self) -> Result<T, Status>;
}

impl<T, R> GrpcHandleError<T> for Result<T, R>
where R: Display
{
    fn internal_error(self) -> Result<T, Status> {
        self.map_err(|err| Status::internal(err.to_string()))
    }
    fn invalid_argument_error(self) -> Result<T, Status> {
        self.map_err(|err| Status::invalid_argument(err.to_string()))
    }
    fn not_found_error(self) -> Result<T, Status> {
        self.map_err(|err| Status::not_found(err.to_string()))
    }
    fn already_exists_error(self) -> Result<T, Status> {
        self.map_err(|err| Status::already_exists(err.to_string()))
    }
    fn permission_denied_error(self) -> Result<T, Status> {
        self.map_err(|err| Status::permission_denied(err.to_string()))
    }
    fn unauthenticated_error(self) -> Result<T, Status> {
        self.map_err(|err| Status::unauthenticated(err.to_string()))
    }
}

// impl<T, S> GrpcHandleError<T> for Result<T, S>
// where S: Debug
// {
//     fn internal_error(self) -> Result<T, Status> {
//         self.map_err(|err| Status::internal(format!("{:?}", err)))
//     }
//     fn invalid_argument_error(self) -> Result<T, Status> {
//         self.map_err(|err| Status::invalid_argument(format!("{:?}", err)))
//     }
//     fn not_found_error(self) -> Result<T, Status> {
//         self.map_err(|err| Status::not_found(format!("{:?}", err)))
//     }
//     fn already_exists_error(self) -> Result<T, Status> {
//         self.map_err(|err| Status::already_exists(format!("{:?}", err)))
//     }
//     fn permission_denied_error(self) -> Result<T, Status> {
//         self.map_err(|err| Status::permission_denied(format!("{:?}", err)))
//     }
//     fn unauthenticated_error(self) -> Result<T, Status> {
//         self.map_err(|err| Status::unauthenticated(format!("{:?}", err)))
//     }
// }
