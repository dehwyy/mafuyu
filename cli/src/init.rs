use tokio::fs::{create_dir_all, File};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::process::Command;
use logger::info;

struct Init;

impl Init {
    pub async fn pnpm_install() {
        let pnpm = crate::internal::os_vars::Executable::get_pnpm();

        info!("Installing pnpm deps...");

        Command::new(pnpm).arg("i").output().await.unwrap();

        info!("Done!")
    }

    pub async fn create_necessary_dirs() {
        create_dir_all(".cdn/static").await.unwrap();
        create_dir_all("libs/grpc/dist").await.unwrap();
    }

    async fn create_rw_file(path: impl AsRef<std::path::Path>) -> File {
        File::options().create_new(true).write(true).read(true).open(path).await.expect("cannot open file")
    }

    pub async fn create_necessary_files() {
        File::create("db.cdn_filenames.redb").await.unwrap();

        {
            let mut env_f = File::options().read(true).open(".env.example").await.expect("cannot open .env.example");
            let mut buf: Vec<u8> = vec!();
            env_f.read_to_end(&mut buf).await.expect("cannot read from .env.example");
            info!("env example: {}", String::from_utf8_lossy(&buf));
            let mut f = Self::create_rw_file(".env").await;
            f.write(&buf).await.expect("cannot write to .env file");
            f.flush().await.expect("cannot flush .env file");
        }
        {
            let mut env_hosts_f = File::options().read(true).open(".env.hosts.example").await.expect("cannot open .env.hosts.example");
            let mut buf: Vec<u8> = vec!();
            env_hosts_f.read_to_end(&mut buf).await.expect("cannot read from .env.hosts.example");
            let mut f = Self::create_rw_file(".env.hosts").await;
            f.write(&buf).await.expect("cannot write to .env.hosts file");
            f.flush().await.expect("cannot flush .env.hosts file");
        }

    }
}

pub async fn init() {
    Init::pnpm_install().await;
    Init::create_necessary_dirs().await;
    Init::create_necessary_files().await;
}