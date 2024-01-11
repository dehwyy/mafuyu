use envconfig::Envconfig;
use super::init_from_file;

#[derive(Envconfig, Default, Debug)]
pub struct Hosts {
    #[envconfig(from = "GATEWAY")]
    pub gateway: String,

    #[envconfig(from = "AUTH")]
    pub auth: String,
}

impl Hosts {
    pub fn new() -> Self {
        init_from_file(".env.hosts")
    }
}
