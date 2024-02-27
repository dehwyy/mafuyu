pub struct CreateProviderPayload {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_url: String,
}

pub struct OAuth2Token {
    pub access_token: String,
    pub refresh_token: Option<String>,
}

pub enum RefreshError {
    NotSupported,
    Internal(String)
}

pub enum OAuth2ProviderName {
    Github,
    Google
}