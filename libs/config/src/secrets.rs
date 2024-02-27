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

    #[envconfig(from = "GOOGLE_CLIENT_ID")]
    pub google_client_id: String,

    #[envconfig(from = "GOOGLE_CLIENT_SECRET")]
    pub google_client_secret: String,

    #[envconfig(from = "GOOGLE_REDIRECT_URL")]
    pub google_redirect_url: String,

    #[envconfig(from = "SENTRY_DSN_RS")]
    pub sentry_dsn: String,

    #[envconfig(from = "ENVIRONMENT", default = "dev")]
    pub environment: String,
}

impl Secrets {
    pub fn new() -> Self {
        init_from_file(".env")
    }
}
