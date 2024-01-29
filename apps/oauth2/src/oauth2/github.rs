use oauth2::{AuthorizationCode, RedirectUrl, TokenResponse, ClientId, ClientSecret, CsrfToken};
use oauth2::{basic::BasicClient, AuthUrl, TokenUrl};
use oauth2::reqwest::async_http_client;
use makoto_logger::{error, info};
use super::*;
use constants::*;

pub struct Github {
  client: BasicClient
}

#[tonic::async_trait]
impl OAuth2Provider for Github {
  fn new(payload: CreateProviderPayload) -> Self {
    // from docs @see https://github.com/ramosbugs/oauth2-rs/blob/main/examples/github_async.rs
    let auth_url = AuthUrl::new("https://github.com/login/oauth/authorize".to_string()).expect("Invalid authorization endpoint URL");
    let token_url = TokenUrl::new("https://github.com/login/oauth/access_token".to_string()).expect("Invalid token endpoint URL");

    let client: BasicClient = BasicClient::new(
      ClientId::new(payload.client_id), Some(ClientSecret::new(payload.client_secret)), auth_url, Some(token_url)
    )
      .set_redirect_uri(RedirectUrl::new(payload.redirect_url).expect("Invalid redirect URL"));

    Self {
      client
    }
  }

  fn create_redirect_url(&self) -> (String, CsrfToken) {
    let (url, csrf_token) =  self.client.authorize_url(CsrfToken::new_random).url();

    (url.to_string(), csrf_token)
  }

  async fn exchange_code_to_token(&self, code: String) -> Result<OAuth2Token, String> {
    let token = self.client
        .exchange_code(AuthorizationCode::new(code))
        .request_async(async_http_client)
        .await.map_err(|err| err.to_string())?;

    Ok(OAuth2Token {
      access_token: token.access_token().secret().to_string(),
      refresh_token: token.refresh_token().map(|token| token.secret().to_string()),
    })
  }


  async fn refresh(&self) -> Result<OAuth2Token, RefreshError> {
    Err(RefreshError::NotSupported)
  }
}
