use tokio::process::Command;
use crate::internal::os_vars::Executable;

struct Grpc;

impl Grpc {
    async fn rust() {
        Command::new("cargo").arg("run")
            .spawn().unwrap().wait_with_output().await.unwrap();
    }

    async fn ts() {
        let pnpm = Executable::get_pnpm();
        let cmd = |files: &str| {
            Command::new(&pnpm).arg("exec")
                .args(["protoc", "-I=protos", "--ts_out=dist", "--ts_opt=generate_dependencies,eslint_disable,ts_nocheck,output_javascript", files]).spawn().unwrap().wait_with_output()
        };

        cmd("protos/*.proto").await.unwrap();
        cmd("protos/api.proto").await.unwrap();

        Command::new(pnpm).arg("ts").output().await.unwrap();
    }

    async fn go() {
        let pnpm = Executable::get_pnpm();
        let cmd = |file: &str| {
            Command::new(&pnpm).arg("exec")
                .args(["protoc", "--go_out=gen", "--go-grpc_out=gen", "--experimental_allow_proto3_optional", "--go_opt=paths=source_relative",  "--go-grpc_opt=paths=source_relative", "--proto_path=protos", file]).spawn().unwrap().wait_with_output()
        };

        cmd("protos/api/mailer.proto").await.unwrap();
    }
}

pub async fn grpc() {

    let mut cwd = std::env::current_dir().unwrap();
    cwd.push("libs/grpc");
    std::env::set_current_dir(cwd).unwrap();

    Grpc::rust().await;
    Grpc::ts().await;
    Grpc::go().await;
}