use tokio::fs::{create_dir_all, File};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::internal::animated::{Animated, Process};
use crate::internal::cmd::Cmd;


struct Init;

impl Init {
    pub async fn pnpm_install() {
        let pnpm = crate::internal::os_vars::Executable::get_pnpm();

        Cmd::tokio_cmd(format!("{pnpm} install")).await.unwrap();
    }

    pub async fn create_necessary_dirs() {
        create_dir_all(".cdn/static").await.unwrap();
        create_dir_all("libs/grpc/dist").await.unwrap();
        create_dir_all("libs/grpc/gen").await.unwrap();
    }

    pub async fn create_necessary_files() {
        const DB_FILENAME: &str = "db.cdn_filenames.redb";
        const ENVFILE: &str = ".env";
        const ENVHOSTSFILE: &str = ".env.hosts";

        let f1= tokio::spawn(async {
            if let Err(_) = File::open(DB_FILENAME).await {
                File::create("db.cdn_filenames.redb").await.unwrap();
            };
            ()
        });

        let f2 = tokio::spawn(async {
            if let Err(_) = File::open(ENVFILE).await {
                let mut env_f = File::options().read(true).open(".env.example").await.unwrap();
                let mut buf: Vec<u8> = vec!();
                env_f.read_to_end(&mut buf).await.unwrap();

                let mut f = Self::create_rw_file(ENVFILE).await;
                f.write(&buf).await.unwrap();
            };
            ()
        });

        let f3 = tokio::spawn(async {
            if let Err(_) = File::open(ENVHOSTSFILE).await {
                let mut env_hosts_f = File::options().read(true).open(".env.hosts.example").await.unwrap();
                let mut buf: Vec<u8> = vec!();
                env_hosts_f.read_to_end(&mut buf).await.unwrap();

                let mut f = Self::create_rw_file(ENVHOSTSFILE).await;
                f.write(&buf).await.unwrap();
            };
            ()
        });

        let _ = tokio::join!(f1, f2, f3);
    }

    async fn create_rw_file(path: impl AsRef<std::path::Path>) -> File {
        File::options().create_new(true).write(true).read(true).open(path).await.expect("cannot open file")
    }
}

pub async fn init() {

    Animated::builder()
        .add(Process {
            func: Init::pnpm_install(),
            on_start: "Installing pnpm...",
            on_end: "Deps installed!",
        })
        .add(Process {
            func: Init::create_necessary_dirs(),
            on_start: "Creating necessary dirs...",
            on_end: "Dirs created!",
        })
        .add(Process {
            func: Init::create_necessary_files(),
            on_start: "Creating necessary files...",
            on_end: "Files created!",
        })
        .invoke().await;
}