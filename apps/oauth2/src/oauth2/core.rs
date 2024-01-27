use serde::Serialize;

pub struct CreateProviderPayload {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_url: String,
    pub scopes: Vec<String>,
}

#[derive(Serialize)]
pub struct OAuth2UserResponse {
    pub id: String,
    pub username: String,
    pub email: Option<String>,
    pub picture: Option<String>
}

#[derive(Serialize)]
pub struct OAuth2Token {
    pub access_token: String,
    pub refresh_token: Option<String>,
}

pub enum OAuth2ProviderName {
    Github,
    Google
}

impl OAuth2ProviderName {
    pub fn from_str(provider_name: &str) -> Option<Self> {
        match provider_name {
            "github" => Some(Self::Github),
            "google" => Some(Self::Google),
            _ => None
        }
    }
}