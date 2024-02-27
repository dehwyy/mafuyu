use oauth2::{RedirectUrl, ClientId, ClientSecret, CsrfToken};
use oauth2::{basic::BasicClient, AuthUrl, TokenUrl, RevocationUrl};
use super::*;

pub struct Google {
    client: BasicClient
}

impl Google {
    pub fn new(payload: CreateProviderPayload) -> Self {
        let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string()).expect("Invalid authorization endpoint URL");
        let token_url = TokenUrl::new("https://oauth2.googleapis.com/token".to_string()).expect("Invalid token endpoint URL");

        let client: BasicClient = BasicClient::new(
            ClientId::new(payload.client_id), Some(ClientSecret::new(payload.client_secret)), auth_url, Some(token_url)
        ).set_redirect_uri(
            RedirectUrl::new(payload.redirect_url).expect("Invalid redirect URL")
        ).set_revocation_uri(
            RevocationUrl::new("https://oauth2.googleapis.com/revoke".to_string())
                .expect("Invalid revocation endpoint URL"),
        );

        Self {
            client
        }
    }
}

#[tonic::async_trait]
impl OAuth2Provider for Google {


    fn get_basic_client(&self) -> &BasicClient {
        &self.client
    }

    fn create_redirect_url(&self) -> (String, CsrfToken) {
        let scopes: Vec<oauth2::Scope> =
            ["https://www.googleapis.com/auth/userinfo.email", "https://www.googleapis.com/auth/userinfo.profile"]
                .iter().map(|s| oauth2::Scope::new(s.to_string())).collect();

        let (url, csrf_token) = self.get_basic_client()
            .authorize_url(CsrfToken::new_random)
            .add_extra_param("access_type", "offline")
            .add_extra_param("include_granted_scopes", "true")
            .add_extra_param("prompt", "select_account")
            .add_scopes(scopes)
            .url();


        (url.to_string(), csrf_token)
    }
}