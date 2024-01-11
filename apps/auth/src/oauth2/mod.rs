use std::time::Duration;

use oauth2::CsrfToken;

mod github;

pub struct CreateProviderPayload {
  pub client_id: String,
  pub client_secret: String,
  pub redirect_url: String,
  pub scopes: Vec<String>,
}

pub struct OAuth2UserResponse {
  pub id: i32,
  pub username: String,
  pub email: String,
  pub picture: String
}

pub struct OAuth2Token {
  pub access_token: String,
  pub refresh_token: Option<String>,
  pub expires_in: Duration
}

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

  pub fn get_provider_by_name(&self, provider_name: &str) -> Option<&impl OAuth2Provider> {
    match provider_name {
      "github" => Some(&self.github),
      _ => None
    }
  }
}

#[tonic::async_trait]
pub trait OAuth2Provider {
  fn new(payload: CreateProviderPayload) -> Self
  where Self: Sized;

  fn create_redirect_url(&self) -> (String, CsrfToken);
  async fn exchange_code_to_token(&self, code: String) -> Result<OAuth2Token, String>;
  async fn get_user_by_token(&self, access_token: String) -> Result<OAuth2UserResponse, String>;
}

pub mod constants {
  pub const GITHUB_PROFILE_URL: &str = "https://api.github.com/user";
}
