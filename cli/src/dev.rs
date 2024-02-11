use std::process::Output;
use std::time::Duration;
use tokio::process::Command;
use tokio::time::sleep;
use makoto_logger::info;


struct Cargo {
    apps: Vec<String>
}

impl Cargo {
    pub fn new() -> Self {
        let apps = vec!(
            "integrations",
            "passport",
            "cdn_node",
            "cdn",
            "user",
            "oauth",
            "tokens",
            "tokens_async",
            "auth",
            "grpc_gateway",
        );

        Self {
            apps: apps.iter().map(|app| app.to_string()).collect()
        }
    }

    pub async fn run(self) {
        info!("Building cargo apps...");

        for app in &self.apps {
            info!("Building app: {}", app);
            Self::build_app(app.clone()).await;
        };
        info!("All cargo apps build success!");


        let mut runtimes = vec!();
        for app in &self.apps {
            info!("Running app: {}", app);

            sleep(Duration::from_secs(2)).await;
            runtimes.push(tokio::spawn(Self::run_app(app.clone())));
        }

        for runtime in runtimes {
            runtime.await.unwrap();
        }
    }

    async fn build_app(app: String) -> Output {
        Command::new("cargo").arg("build")
            .args(["-p", &app])
            .output().await.unwrap()
    }

    async fn run_app(app: String) -> Output {
        Command::new("cargo").arg("run")
            .args(["-p", &app, "-q", "--color", "always"])
            .spawn().unwrap().wait_with_output()
            .await.unwrap()
    }
}

struct Others;

impl Others {
    pub async fn caddy_run() {
        info!("Starting Caddy!");
        Command::new("caddy").arg("run")
            .spawn().unwrap().wait_with_output().await.unwrap();
    }

    pub async fn docker_compose_up() {
        info!("Starting Docker!");
        Command::new("docker-compose")
            .args(["-f", "docker-compose.dev.yml"])
            .arg("up")
            .arg("-d")
            .output().await.unwrap();
    }
}

pub async fn dev() {
    Others::docker_compose_up().await;

    let cargo_runtime = tokio::spawn(Cargo::new().run());
    let caddy_runtime = tokio::spawn(Others::caddy_run());

    let runtimes = [cargo_runtime, caddy_runtime];

    for runtime in runtimes {
        runtime.await.unwrap();
    }
}

