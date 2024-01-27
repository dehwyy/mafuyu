pub mod router ;
pub mod jwt;
pub mod repo;
pub mod tools;

pub type AsyncResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;
