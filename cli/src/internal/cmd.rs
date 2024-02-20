use std::process::Output;
use tokio::process::Command;


pub struct Cmd;

impl Cmd {
    pub async fn tokio_cmd(command: String, wait_finish: bool) -> Result<Output, tokio::io::Error> {
        let mut iter = command.split(" ");
        let program = iter.next().unwrap();
        let args = iter.collect::<Vec<_>>();

        match wait_finish {
            true => Command::new(program).args(args).output().await,
            false => Command::new(program).args(args).spawn().unwrap().wait_with_output().await,
        }
    }
}