use envconfig::Envconfig as EnvConfigTrait;
use dotenv;
use std::env::current_dir as get_cwd;

pub mod constants;
pub mod hosts;
pub mod db;
pub mod secrets;

fn init_from_file<T>(filename: &str) -> T
where T: Default + EnvConfigTrait
{

    for i in 0..3 {
        let mut env_path = get_cwd().unwrap();

        for _ in 0..i {
            env_path.pop();
        }

        let _ = dotenv::from_path(env_path.join(filename));
    }

    T::init_from_env().unwrap_or_default()
}
