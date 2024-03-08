use tokio::process::Command;
use tokio::time::{sleep, Duration};
use logger::info;
use crate::internal::animated::{Animated, Process};

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
        let mut multi = Animated::builder();
        for (i, build_cmd) in self.apps_build_commands.iter().enumerate().collect::<Vec<_>>() {
            let app = self.apps.get(i).unwrap().service.clone();
            let cmd = build_cmd.clone();

            multi = multi.add(Process{
                func: Box::pin(async move {Cmd::tokio_cmd(cmd).await.map(|_| {}).unwrap() }),
                on_start: format!("Building app: {app}"),
                on_end: format!("Built app: {app}"),
            });
        }

        multi.invoke().await;

        let mut runtimes = vec!();
        for (i, exec_cmd) in self.apps_exec_commands.iter().enumerate().collect::<Vec<_>>() {
            let app: &App  = self.apps.get(i).unwrap();
            info!("Running app: {}", app.service);

            sleep(Duration::from_secs(2)).await;
            runtimes.push(tokio::spawn(Cmd::tokio_cmd_with_output(exec_cmd.clone())));
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
    pub async fn caddy_start() {
        info!("Starting Caddy!");
        Command::new("caddy").arg("start")
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
    Others::caddy_start().await;

    let apps: Apps = Apps::new();
    apps.exec().await;
}

