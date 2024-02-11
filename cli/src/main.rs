mod dev;

use clap::{Parser, Subcommand, Args};

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
    Dev
}


#[tokio::main]
async fn main() {
    makoto_logger::Logger::init().unwrap();

    let cli = Cli::parse();
    match &cli.command {
            Command::Dev => dev::dev().await
    };
}