mod github;
mod core;
mod google;

use oauth2::basic::BasicClient;
pub use self::core::{OAuth2ProviderName, RefreshError};

use oauth2::{AuthorizationCode, CsrfToken, RefreshToken, TokenResponse};
use oauth2::reqwest::async_http_client;
use self::core::*;


pub struct OAuth2 {
  github:  Box<dyn OAuth2Provider>,
  google: Box<dyn OAuth2Provider>
}

impl OAuth2 {
  pub fn new() -> Self {
    let secrets = makoto_config::secrets::Secrets::new();

    Self {
      github: Box::new(github::Github::new(CreateProviderPayload{
        client_id: secrets.github_client_id,
        client_secret: secrets.github_client_secret,
        redirect_url: secrets.github_redirect_url,
      })),
      google: Box::new(google::Google::new(CreateProviderPayload{
        client_id: secrets.google_client_id,
        client_secret: secrets.google_client_secret,
        redirect_url: secrets.google_redirect_url,
      }))
    }
  }

  pub fn get_provider(&self, provider: OAuth2ProviderName) -> &Box<dyn OAuth2Provider> {
    match provider {
      OAuth2ProviderName::Github => &self.github,
      OAuth2ProviderName::Google => &self.google
    }

  }
}

#[tonic::async_trait]
pub trait OAuth2Provider: Send + Sync {
  fn get_basic_client(&self) -> &BasicClient;

  fn create_redirect_url(&self) -> (String, CsrfToken) {
    let (url, csrf_token) = self.get_basic_client().authorize_url(CsrfToken::new_random).url();
    (url.to_string(), csrf_token)
  }

  async fn exchange_code_to_token(&self, code: String) -> Result<OAuth2Token, String> {
    let token = self.get_basic_client()
        .exchange_code(AuthorizationCode::new(code))
        .request_async(async_http_client)
        .await.map_err(|err| err.to_string())?;

    Ok(OAuth2Token {
      access_token: token.access_token().secret().to_string(),
      refresh_token: token.refresh_token().map(|token| token.secret().to_string()),
    })
  }

  async fn refresh(&self, refresh_token: Option<String>) -> Result<OAuth2Token, RefreshError> {
    match refresh_token.map(|t| RefreshToken::new(t)) {
      Some(refresh_token) => {
        let token = self.get_basic_client()
            .exchange_refresh_token(&refresh_token)
            .request_async(async_http_client)
            .await.map_err(|err| RefreshError::Internal(err.to_string()))?;

        Ok(OAuth2Token {
          access_token: token.access_token().secret().to_string(),
          refresh_token: token.refresh_token().map(|token| token.secret().to_string()),
        })
      },
      None => Err(RefreshError::NotSupported)
    }
  }
}
