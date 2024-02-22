use tokio::process::Command;
use tokio::time::{sleep, Duration};
use makoto_logger::info;

use crate::internal::cmd::Cmd;

#[derive(Clone)]
enum Lang {
    GO,
    RUST
}

#[derive(Clone)]
struct App {
    pub service: String,
    lang: Lang
}

struct Apps {
    apps: Vec<App>,
    apps_build_commands: Vec<String>,
    apps_exec_commands: Vec<String>,
}

impl Apps {
    pub fn new() -> Self {
        let cb = Self::cargo_build;
        let c = Self::cargo_run;
        let gb = Self::go_build;
        let g = Self::go_run;

        let apps: Vec<App> = vec!(
            App { service: "mailer".to_string(), lang: Lang::GO },
            App { service: "mailer-async".to_string(), lang: Lang::GO },
            App { service: "integrations".to_string(), lang: Lang::RUST },
            App { service: "passport".to_string(), lang: Lang::RUST },
            App { service: "cdn_node".to_string(), lang: Lang::RUST },
            App { service: "cdn".to_string(), lang: Lang::RUST },
            App { service: "oauth".to_string(), lang: Lang::RUST },
            App { service: "user".to_string(), lang: Lang::RUST },
            App { service: "tokens".to_string(), lang: Lang::RUST },
            App { service: "tokens_async".to_string(), lang: Lang::RUST },
            App { service: "auth".to_string(), lang: Lang::RUST },
            App { service: "authorization".to_string(), lang: Lang::RUST },
            App { service: "grpc_gateway".to_string(), lang: Lang::RUST },
        );

        Self {
            apps: apps.clone(),
            apps_build_commands: apps.iter().map(|app| {
                match app.lang {
                    Lang::GO => gb(&app.service),
                    Lang::RUST => cb(&app.service),
                }
            }).collect(),
            apps_exec_commands: apps.iter().map(|app| {
                match app.lang {
                    Lang::GO => g(&app.service),
                    Lang::RUST => c(&app.service),
                }
            }).collect(),
        }
    }

    pub async fn exec(self) {
        for (i, build_cmd) in self.apps_build_commands.iter().enumerate().collect::<Vec<_>>() {
            let app: &App  = self.apps.get(i).unwrap();
            info!("Building app: {}", app.service);
            Cmd::tokio_cmd(build_cmd.clone(), true).await.unwrap();
        }

        let mut runtimes = vec!();
        for (i, exec_cmd) in self.apps_exec_commands.iter().enumerate().collect::<Vec<_>>() {
            let app: &App  = self.apps.get(i).unwrap();
            info!("Running app: {}", app.service);

            sleep(Duration::from_secs(2)).await;
            runtimes.push(tokio::spawn(Cmd::tokio_cmd(exec_cmd.clone(), false)));
        }

        for runtime in runtimes {
            runtime.await.unwrap().unwrap();
        }
    }

    fn cargo_run(app: &str) -> String {
        format!("cargo run -p {} -q --color always", app)
    }

    fn cargo_build(app: &str) -> String {
        format!("cargo build -p {}", app)
    }

    fn go_run(app: &str) -> String {
        format!("go run apps/{}/main.go", app)
    }

    fn go_build(app :&str) -> String {
        format!("go build apps/{}/main.go", app)
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

    pub async fn migrate_db() {
        info!("Migrating db...!");

        std::env::set_var("DATABASE_DSN", "host=localhost user=postgres password=postgres dbname=postgres port=54321 sslmode=disable");
        Command::new("go")
            .args(["run", "libs/db/main.go", "migrate"])
            .output().await.unwrap();

        info!("Migrating db succeed!");
    }
}

pub async fn dev() {
    Others::docker_compose_up().await;
    Others::migrate_db().await;

    let apps: Apps = Apps::new();
    let apps_runtime = tokio::spawn(apps.exec());

    let caddy_runtime = tokio::spawn(Others::caddy_run());

    let runtimes = [apps_runtime, caddy_runtime];

    for runtime in runtimes {
        runtime.await.unwrap();
    }
}

