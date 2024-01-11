use envconfig::Envconfig;
use super::init_from_file;

#[derive(Envconfig, Default, Debug)]
pub struct Secrets {
    #[envconfig(from = "JWT_SECRET")]
    pub jwt_secret: Option<String>,

    #[envconfig(from = "GITHUB_CLIENT_ID")]
    pub github_client_id: String,

    #[envconfig(from = "GITHUB_CLIENT_SECRET")]
    pub github_client_secret: String,

    #[envconfig(from = "GITHUB_REDIRECT_URL")]
    pub github_redirect_url: String,
}

impl Secrets {
    pub fn new() -> Self {
        init_from_file(".env")
    }
}
