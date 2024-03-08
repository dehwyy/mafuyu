use std::process::Output;
use tokio::process::Command;


pub struct Cmd;

impl Cmd {
    async fn tokio_cmd_core(command: String, with_output: bool) -> Result<Output, tokio::io::Error> {
        let mut iter = command.split(" ");
        let program = iter.next().unwrap();
        let args = iter.collect::<Vec<_>>();

        match with_output {
            true => Command::new(program).args(args).spawn().unwrap().wait_with_output().await,
            false => Command::new(program).args(args).output().await,
        }
    }

    pub async fn tokio_cmd(command: String) -> Result<Output, tokio::io::Error> {
       Self::tokio_cmd_core(command, false).await
    }

    pub async fn tokio_cmd_with_output(command: String) -> Result<Output, tokio::io::Error> {
        Self::tokio_cmd_core(command, true).await
    }
}