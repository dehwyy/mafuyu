use tokio::fs::{create_dir_all, File};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use yomi::anim::{AnimatedProcess, Animation};
use yomi::CommandExecutor;

use crate::internal::executable::Executable;

struct Init;

impl Init {
    async fn pnpm_install() {
        CommandExecutor::execute(format!("{pnpm} install", pnpm = Executable::Pnpm))
            .await
            .unwrap();
    }

    async fn create_necessary_dirs() {
        // TODO? maybe async?
        create_dir_all(".cdn/static").await.unwrap();
        create_dir_all("libs/grpc/dist").await.unwrap();
        create_dir_all("libs/grpc/gen").await.unwrap();
    }

    async fn create_necessary_files() {
        const DB_FILENAME: &str = "db.cdn_filenames.redb";
        const ENVFILE: &str = ".env";
        const ENVHOSTSFILE: &str = ".env.hosts";

        let redb_db_file = tokio::spawn(async {
            // If file could not be opened (doesn't exist) -> create it.
            if let Err(_) = File::open(DB_FILENAME).await {
                File::create("db.cdn_filenames.redb").await.unwrap();
            };
        });

        // TODO: maybe refactor?
        let env_file = tokio::spawn(async {
            if let Err(_) = File::open(ENVFILE).await {
                let mut env_f = File::options()
                    .read(true)
                    .open(".env.example")
                    .await
                    .unwrap();
                let mut buf: Vec<u8> = vec![];
                env_f.read_to_end(&mut buf).await.unwrap();

                let mut f = Self::create_rw_file(ENVFILE).await;
                f.write(&buf).await.unwrap();
            };
        });

        let env_hosts_file = tokio::spawn(async {
            if let Err(_) = File::open(ENVHOSTSFILE).await {
                let mut env_hosts_f = File::options()
                    .read(true)
                    .open(".env.hosts.example")
                    .await
                    .unwrap();
                let mut buf: Vec<u8> = vec![];
                env_hosts_f.read_to_end(&mut buf).await.unwrap();

                let mut f = Self::create_rw_file(ENVHOSTSFILE).await;
                f.write(&buf).await.unwrap();
            };
            ()
        });

        let _ = tokio::join!(redb_db_file, env_file, env_hosts_file);
    }

    async fn create_rw_file(path: impl AsRef<std::path::Path>) -> File {
        File::options()
            .create_new(true)
            .write(true)
            .read(true)
            .open(path)
            .await
            .expect("cannot open file")
    }
}

pub async fn init() {
    Animation::builder()
        .add(
            AnimatedProcess::new(Init::pnpm_install())
                .set_text_during_execution("Installing pnpm...")
                .set_text_after_execution("Depenencies were installed!"),
        )
        .add(
            AnimatedProcess::new(Init::create_necessary_dirs())
                .set_text_during_execution("Creating necessary dirs...")
                .set_text_after_execution("Dirs were created!"),
        )
        .add(
            AnimatedProcess::new(Init::create_necessary_files())
                .set_text_during_execution("Creating necessary files...")
                .set_text_after_execution("Files were created!"),
        )
        .invoke_parallel()
        .await;
}
