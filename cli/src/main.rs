mod dev;
mod grpc;
mod init;
mod internal;

use logger::{Logger, LoggerConfig};

use yomi::prelude::{Parser, Subcommand};
use yomi::Cli;

#[derive(Subcommand)]
enum Command {
    /// Run Mafuyu's apps in dev mode.
    #[command(name = "dev")]
    Dev,

    /// Gen GRPC files
    #[command(name = "grpc")]
    Grpc,

    /// Init MaFuYu
    #[command(name = "init")]
    Init,
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
    match cli.get() {
        Command::Dev => dev::dev().await,
        Command::Grpc => grpc::grpc().await,
        Command::Init => init::init().await,
    };
}
