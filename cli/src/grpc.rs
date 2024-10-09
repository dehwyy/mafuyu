use std::env::{current_dir, set_current_dir};

use yomi::anim::{AnimatedProcess, Animation};
use yomi::CommandExecutor;

use crate::internal::executable::Executable;

struct Grpc;

impl Grpc {
    async fn rust() {
        CommandExecutor::execute("cargo run").await.unwrap();
    }

    async fn ts() {
        let cmd = |f: &str| {
            CommandExecutor::execute(
                format!(
                    "{pnpm} exec protoc -I=protos --ts_out=dist --ts_opt=generate_dependencies,eslint_disable,ts_nocheck,output_javascript {f}",
                    pnpm = Executable::Pnpm
                )
            )
        };

        cmd("protos/*.proto").await.unwrap();
        cmd("protos/api.proto").await.unwrap();

        CommandExecutor::execute(format!("{pnpm} ts", pnpm = Executable::Pnpm))
            .await
            .unwrap();
    }

    async fn go() {
        let cmd = |f: &str| {
            CommandExecutor::execute(
                format!(
                    "{pnpm} exec protoc -I=protos --go_out=gen --go_opt=paths=source_relative --go-grpc_out=gen --go-grpc_opt=paths=source_relative --proto_path=protos {f}",
                    pnpm = Executable::Pnpm
                )
            )
        };

        cmd("protos/api/mailer.proto").await.unwrap();
    }
}

pub async fn grpc() {
    let mut cwd = current_dir().unwrap();
    cwd.push("libs/grpc");

    set_current_dir(cwd).unwrap();

    Animation::builder()
        .add(
            AnimatedProcess::new(Grpc::rust())
                .set_text_during_execution("Generating .rs files from .proto...")
                .set_text_after_execution("Generated .rs!"),
        )
        .add(
            AnimatedProcess::new(Grpc::ts())
                .set_text_during_execution("Generating .ts files from .proto...")
                .set_text_after_execution("Generated .ts!"),
        )
        .add(
            AnimatedProcess::new(Grpc::go())
                .set_text_during_execution("Generating .go files from .proto...")
                .set_text_after_execution("Generated .go!"),
        )
        .invoke_parallel()
        .await;
}
