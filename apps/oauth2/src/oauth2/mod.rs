mod github;
mod core;
pub use self::core::{OAuth2ProviderName, RefreshError};

use oauth2::CsrfToken;
use self::core::*;


pub struct OAuth2{
  github: github::Github
}

impl OAuth2 {
  pub fn new() -> Self {
    let secrets = makoto_config::secrets::Secrets::new();

    Self {
      github: github::Github::new(CreateProviderPayload{
        client_id: secrets.github_client_id,
        client_secret: secrets.github_client_secret,
        redirect_url: secrets.github_redirect_url,
        scopes: vec!()
      })
    }
  }

  pub fn get_provider(&self, provider: OAuth2ProviderName) -> &impl OAuth2Provider {
    match provider {
      OAuth2ProviderName::Github => &self.github,
      OAuth2ProviderName::Google => &self.github // TODO
    }
  }
}

#[async_trait::async_trait]
pub trait OAuth2Provider {
  fn new(payload: CreateProviderPayload) -> Self;

  fn create_redirect_url(&self) -> (String, CsrfToken);
  async fn exchange_code_to_token(&self, code: String) -> Result<OAuth2Token, String>;
  async fn get_user_by_token(&self, access_token: String) -> Result<OAuth2UserResponse, String>;
  async fn refresh(&self) -> Result<OAuth2Token, RefreshError>;
}

pub mod constants {
  pub const GITHUB_PROFILE_URL: &str = "https://api.github.com/user";
}
