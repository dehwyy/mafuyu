use oauth2::{RedirectUrl, ClientId, ClientSecret};
use oauth2::{basic::BasicClient, AuthUrl, TokenUrl};
use super::*;

pub struct Github {
  client: BasicClient
}

impl Github {
  pub fn new(payload: CreateProviderPayload) -> Self {
    // from docs @see https://github.com/ramosbugs/oauth2-rs/blob/main/examples/github_async.rs
    let auth_url = AuthUrl::new("https://github.com/login/oauth/authorize".to_string()).expect("Invalid authorization endpoint URL");
    let token_url = TokenUrl::new("https://github.com/login/oauth/access_token".to_string()).expect("Invalid token endpoint URL");

    let client: BasicClient = BasicClient::new(
      ClientId::new(payload.client_id), Some(ClientSecret::new(payload.client_secret)), auth_url, Some(token_url)
    ).set_redirect_uri(
      RedirectUrl::new(payload.redirect_url).expect("Invalid redirect URL")
    );

    Self {
      client
    }
  }
}

#[tonic::async_trait]
impl OAuth2Provider for Github {
  fn get_basic_client(&self) -> &BasicClient {
    &self.client
  }
}
