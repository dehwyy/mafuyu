use tokio::process::Command;

pub struct Grpc;

impl Grpc {
    async fn rust() {
        Command::new("cargo").arg("run")
            .spawn().unwrap().wait_with_output().await.unwrap();
    }

    async fn ts() {
        let cmd = |files: &str| {
            Command::new("pnpm").arg("exec")
                .args(["protoc", "-I=protos", "--ts_out=dist", "--ts_opt=generate_dependencies,eslint_disable,ts_nocheck,output_javascript", files]).spawn().unwrap().wait_with_output()
        };

        cmd("protos/*.proto").await.unwrap();
        cmd("protos/api.proto").await.unwrap();
    }
}

pub async fn grpc() {


    let mut cwd = std::env::current_dir().unwrap();
    cwd.push("libs/grpc");
    std::env::set_current_dir(cwd).unwrap();

    Grpc::rust().await;
    Grpc::ts().await;
}