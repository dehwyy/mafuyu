pub mod pkg;

//
use tonic::{Response, Status};

//
pub type Result<T> = std::result::Result<Response<T>, Status>;
