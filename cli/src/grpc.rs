use crate::internal::animated::{Animated, Process};
use crate::internal::cmd::Cmd;
use crate::internal::os_vars::Executable;

struct Grpc;

impl Grpc {
    async fn rust() {
        Cmd::tokio_cmd("cargo run".to_owned()).await.unwrap();
    }

    async fn ts() {
        let pnpm = Executable::get_pnpm();
        let cmd = |files: &str| {
            Cmd::tokio_cmd(format!("{} exec protoc -I=protos --ts_out=dist --ts_opt=generate_dependencies,eslint_disable,ts_nocheck,output_javascript {}", &pnpm, files))
        };

        cmd("protos/*.proto").await.unwrap();
        cmd("protos/api.proto").await.unwrap();

        Cmd::tokio_cmd(format!("{pnpm} ts")).await.unwrap();
    }

    async fn go() {
        let pnpm = Executable::get_pnpm();
        let cmd = |file: &str| {
            Cmd::tokio_cmd(format!("{} exec protoc -I=protos --go_out=gen --go_opt=paths=source_relative --go-grpc_out=gen --go-grpc_opt=paths=source_relative --proto_path=protos {}", &pnpm, file))
        };

        cmd("protos/api/mailer.proto").await.unwrap();
    }
}

pub async fn grpc() {

    let mut cwd = std::env::current_dir().unwrap();
    cwd.push("libs/grpc");
    std::env::set_current_dir(cwd).expect("cannot set cwd");

    Animated::builder()
        .add(Process {
            func: Grpc::rust(),
            on_start: "Generating .rs files from .proto...",
            on_end: "Generated .rs!",
        }).add(Process {
            func: Grpc::ts(),
            on_start: "Generating .ts files from .proto...",
            on_end: "Generated .ts!",
        }).add(Process {
            func: Grpc::go(),
            on_start: "Generating .go files from .proto...",
            on_end: "Generated .go!",
        }).invoke().await;
}