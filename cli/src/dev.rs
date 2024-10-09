use std::env;
use std::fmt::Display;

use logger::info;
use tokio::join;
use tokio::time::{sleep, Duration};

use yomi::anim::{AnimatedProcess, Animation};
use yomi::CommandExecutor;

#[derive(Clone)]
enum Lang {
    Go,
    Rust,
}

#[derive(Clone)]
struct App {
    pub service: String,
    lang: Lang,
}

impl App {
    pub fn new(service: impl Display, lang: Lang) -> Self {
        Self {
            service: service.to_string(),
            lang,
        }
    }
}

/// Microservice apps.
struct Apps {
    apps: Vec<App>,
    apps_build: Vec<String>,
    apps_run: Vec<String>,
}

impl Apps {
    pub fn new() -> Self {
        use Lang::{Go, Rust};

        let apps = vec![
            App::new("mailer", Go),
            App::new("mailer-async", Go),
            App::new("integrations", Rust),
            App::new("passport", Rust),
            App::new("cdn_node", Rust),
            App::new("cdn", Rust),
            App::new("oauth", Rust),
            App::new("user", Rust),
            App::new("tokens", Rust),
            App::new("tokens_async", Rust),
            App::new("auth", Rust),
            App::new("authorization", Rust),
            App::new("grpc_gateway", Rust),
        ];

        let apps_build = apps
            .iter()
            .map(|app| match app.lang {
                Go => format!("go build apps/{}/main.go", app.service),
                Rust => format!("cargo build -p {}", app.service),
            })
            .collect::<Vec<_>>();

        let apps_run = apps
            .iter()
            .map(|app| match app.lang {
                Go => format!("go run apps/{}/main.go", app.service),
                Rust => format!("cargo run -p {}", app.service),
            })
            .collect::<Vec<_>>();

        Self {
            apps,
            apps_build,
            apps_run,
        }
    }

    async fn run(self) {
        let mut multi = Animation::builder();
        for (i, build_command) in self.apps_build.iter().enumerate() {
            let build_command = build_command.clone();
            let app_name = &self.apps[i].service;

            multi = multi.add(
                AnimatedProcess::new(async move {
                    CommandExecutor::execute(build_command).await.unwrap();
                })
                .text_on_execution(format!("Building app: {app_name}"))
                .text_after_execution(format!("Built app: {app_name}")),
            );
        }

        multi.invoke_parallel().await;

        let mut runtimes = vec![];
        for (i, run_command) in self.apps_run.iter().enumerate() {
            let app = &self.apps[i];
            info!("Running app: {}", app.service);

            // ???
            sleep(Duration::from_secs(2)).await;

            runtimes.push(tokio::spawn(CommandExecutor::execute_non_blocking_output(
                run_command.clone(),
            )));
        }

        for runtime in runtimes {
            runtime.await.unwrap().unwrap();
        }
    }
}

/// `Caddy` proxy.
async fn start_caddy() {
    CommandExecutor::execute("caddy start")
        .await
        .expect("failed to string `caddy`.");
}

/// `DockerCompose` development environment.
async fn start_docker() {
    CommandExecutor::execute("docker compose -f docker-compose.dev.yml up -d")
        .await
        .unwrap();
}

/// Database migrations.
async fn migrate_database() {
    let database_url = makoto_config::db::Database::new()
        .database_dsn
        .expect("cannot get database_url from config!");
    env::set_var("DATABASE_DSN", database_url);

    CommandExecutor::execute("go run libs/db/main.go migrate")
        .await
        .unwrap();
}

pub async fn dev() {
    Animation::builder()
        .add(
            AnimatedProcess::new(async { start_docker().await })
                .text_on_execution("Starting Docker...")
                .text_after_execution("Docker started!"),
        )
        .add(
            AnimatedProcess::new(async { migrate_database().await })
                .text_on_execution("Migrating db...")
                .text_after_execution("Db successfully migrated!"),
        )
        .invoke_sequentially("Docker was started!".to_string())
        .await;

    let caddy_runtime = tokio::spawn(start_caddy());

    let apps = Apps::new();

    let _ = join!(caddy_runtime, apps.run());
}
