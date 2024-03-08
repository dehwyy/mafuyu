mod dev;
mod grpc;
mod internal;
mod init;

use clap::{Parser, Subcommand};
use logger::{Logger, LoggerConfig};

#[derive(Parser)]
#[command(version,about, propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Command
}

#[derive(Subcommand)]
enum Command {
    /// Run Mafuyu's apps in dev mode.
    #[command(name="dev")]
    Dev,

    /// Gen GRPC files
    #[command(name="grpc")]
    Grpc,

    /// Init MaFuYu
    #[command(name = "init")]
    Init
}


#[tokio::main]
async fn main() {
    Logger::with_config(LoggerConfig {
        with_file: false,
        with_sentry: false,
        with_target: false,
        with_line_number: false,
        ..Default::default()
    });

    let cli = Cli::parse();
    match &cli.command {
            Command::Dev => dev::dev().await,
            Command::Grpc => grpc::grpc().await,
            Command::Init => init::init().await
    };
}