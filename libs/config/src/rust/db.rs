use envconfig::Envconfig;
use super::init_from_file;

#[derive(Envconfig, Default, Debug)]
pub struct Database {
    #[envconfig(from = "DATABASE_URL")]
    pub database_url: Option<String>,

    #[envconfig(from = "DATABASE_TEST_URL")]
    pub database_test_url: Option<String>,
}

impl Database {
    pub fn new() -> Self {
        init_from_file(".env")
    }
}
