pub mod errors;
mod errors_impl;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
