mod traits;
#[cfg(feature = "rpc")]
mod tonic;
#[cfg(feature = "db")]
mod repo;
#[cfg(feature = "nats")]
mod nats;

pub mod prelude {
    pub use super::traits::{ResultedOption,HandleError};

    #[cfg(feature = "db")]
    pub use crate::errors::repo::RepositoryError;
}


