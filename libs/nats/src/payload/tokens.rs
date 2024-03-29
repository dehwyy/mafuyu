use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ClearTokensRequest {
    pub user_id: String,
    pub access_token: Option<String>
}

pub mod subject {
    pub const CLEAR_TOKENS: &str = "tokens.do.clear_tokens";
}