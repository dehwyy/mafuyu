use tokio::fs::{self, create_dir_all, File};

use yomi::anim::{AnimatedProcess, Animation};
use yomi::CommandExecutor;

use crate::internal::executable::Executable;

const DB_FILENAME: &str = "db.cdn_filenames.redb";

const ENVFILE: &str = ".env";
const ENVFILE_EXAMPLE: &str = ".env.example";

const ENVHOSTSFILE: &str = ".env.hosts";
const ENVHOSTSFILE_EXAMPLE: &str = ".env.hosts.example";

async fn copy_if_not_exists<S: AsRef<std::path::Path> + Copy>(from: S, to: S) {
    if !fs::try_exists(to).await.unwrap() {
        fs::copy(from, to).await.unwrap();
    };
}

struct Init;

impl Init {
    async fn install_deps() {
        CommandExecutor::execute(format!("{} install", Executable::Bun))
            .await
            .unwrap();
    }

    async fn create_necessary_dirs() {
        let _ = tokio::join!(
            create_dir_all(".cdn/static"),
            create_dir_all("libs/grpc/dist"),
            create_dir_all("libs/grpc/gen"),
        );
    }

    async fn create_necessary_files() {
        let redb_db_file = tokio::spawn(async {
            if !fs::try_exists(DB_FILENAME).await.unwrap() {
                File::create(DB_FILENAME).await.unwrap();
            };
        });

        let env_file = tokio::spawn(copy_if_not_exists(ENVFILE_EXAMPLE, ENVFILE));

        let env_hosts_file = tokio::spawn(copy_if_not_exists(ENVHOSTSFILE_EXAMPLE, ENVHOSTSFILE));

        let _ = tokio::join!(redb_db_file, env_file, env_hosts_file);
    }
}

pub async fn init() {
    Animation::builder()
        .add(
            AnimatedProcess::new(Init::install_deps())
                .text_on_execution("Installing dependencies...")
                .text_after_execution("Depenencies were installed!"),
        )
        .add(
            AnimatedProcess::new(Init::create_necessary_dirs())
                .text_on_execution("Creating necessary dirs...")
                .text_after_execution("Dirs were created!"),
        )
        .add(
            AnimatedProcess::new(Init::create_necessary_files())
                .text_on_execution("Creating necessary files...")
                .text_after_execution("Files were created!"),
        )
        .invoke_parallel()
        .await;
}
