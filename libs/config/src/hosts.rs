use envconfig::Envconfig;
use super::init_from_file;

#[derive(Envconfig, Default, Debug)]
pub struct Hosts {
    #[envconfig(from = "GATEWAY")]
    pub gateway: String,

    #[envconfig(from = "AUTH")]
    pub auth: String,

    #[envconfig(from = "AUTHORIZATION")]
    pub authorization: String,

    #[envconfig(from = "OAUTH2")]
    pub oauth2: String,

    #[envconfig(from = "TOKENS")]
    pub tokens: String,

    #[envconfig(from = "PASSPORT")]
    pub passport: String,

    #[envconfig(from = "USER_RPC")]
    pub user: String,

    #[envconfig(from = "INTEGRATIONS")]
    pub integrations: String,

    #[envconfig(from = "CDN_RPC")]
    pub cdn_rpc: String,

    #[envconfig(from = "CDN_API")]
    pub cdn_api: String,

    #[envconfig(from = "MAILER_RPC")]
    pub mailer_rpc: String,
}

impl Hosts {
    pub fn new() -> Self {
        init_from_file(".env.hosts")
    }
}
