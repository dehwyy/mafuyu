use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ClearTokensRequest {
    pub user_id: String,
    pub access_token: Option<String>
}