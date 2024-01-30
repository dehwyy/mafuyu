use envconfig::Envconfig;
use super::init_from_file;

#[derive(Envconfig, Default, Debug)]
pub struct Hosts {
    #[envconfig(from = "GATEWAY")]
    pub gateway: String,

    #[envconfig(from = "AUTH")]
    pub auth: String,

    #[envconfig(from = "OAUTH2")]
    pub oauth2: String,

    #[envconfig(from = "TOKENS")]
    pub tokens: String,

    #[envconfig(from = "PASSPORT")]
    pub passport: String,

    #[envconfig(from = "INTEGRATIONS")]
    pub integrations: String
}

impl Hosts {
    pub fn new() -> Self {
        init_from_file(".env.hosts")
    }
}
