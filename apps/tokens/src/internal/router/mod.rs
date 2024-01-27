use std::fmt::Formatter;
use makoto_logger::error;

pub mod core;
pub mod jetstream;

pub mod payload {
    use serde::{Deserialize, Serialize};
    use crate::internal::router::RouterError;

    #[derive(Deserialize, Serialize)]
    pub struct TokenValidationPayload {
        pub access_token: String
    }

    #[derive(Deserialize, Serialize)]
    pub struct MakeTokensPayload {
        pub user_id: String,
        pub provider: Option<String>
    }

    #[derive(Deserialize, Serialize)]
    pub struct ClearTokensPayload {
        pub user_id: String,
        pub access_token: Option<String>
    }

    pub fn get_payload<'a, T>(v: &'a [u8]) -> Result<T, RouterError>
        where T: Deserialize<'a>
    {
        serde_json::from_slice::<T>(v).map_err(|err| {
            RouterError::WrongPayload(err.to_string())
        })
    }
}

mod response {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize)]
    pub struct CommonResponse {
        pub data: String,
        pub error: Option<String>
    }

    #[derive(Deserialize, Serialize)]
    pub struct ErrorResponse {
        pub error: String
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct TokenValidationResponse {
        pub valid: bool,
    }

    #[derive(Deserialize, Serialize)]
    pub struct MakeTokensResponse {
        pub access_token: String,
        pub refresh_token: String
    }

}

pub enum RouterError {
    Internal(String),
    WrongPayload(String),
    CannotRespond(String),
    NotFound(String),
    Local(String) //
}

impl std::fmt::Display for RouterError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {

        let s = match self {
            RouterError::Internal(err) => format!("[internal router error]: {err}"),
            RouterError::CannotRespond(err) => format!("[cannot respond to subject]: {err}"),
            RouterError::WrongPayload(err) => format!("[wrong payload] {err}"),
            RouterError::NotFound(err) => format!("[not found] {}", err),
            RouterError::Local(err) => format!("[local error] {err}")
        };

        write!(f, "{}", s)
    }
}

impl RouterError {
    pub fn log(&self) {

        error!("[router error] {s}", s=self.to_string());
    }
}