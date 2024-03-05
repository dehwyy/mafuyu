#[cfg(feature = "tower")]
pub mod tower;

use tracing_subscriber::prelude::*;
use tracing_subscriber::fmt::time::ChronoLocal;
pub use tracing::*;

#[derive(Clone)]
enum  LoggerMode {
    Trace,
    Default,
    Production
}

impl LoggerMode {
    fn new(env: String) -> Self {
        match env.as_str() {
            "trace" => LoggerMode::Trace,
            "debug" => LoggerMode::Default,
            "dev" => LoggerMode::Default,
            "production" => LoggerMode::Production,
            _ => LoggerMode::Default
        }
    }
}

impl Into<Level> for LoggerMode {
    fn into(self) -> Level {
        match self {
            LoggerMode::Trace => Level::TRACE,
            LoggerMode::Default => Level::INFO,
            LoggerMode::Production => Level::ERROR
        }
    }
}

pub struct Logger;

impl Logger {
    pub fn new(environment: String) {
        let mode = LoggerMode::new(environment);
        let level: Level = mode.clone().into();

        let fmt_subscriber = tracing_subscriber::FmtSubscriber::builder()
            .with_target(true)
            .with_file(true)
            .with_line_number(true)
            .with_ansi(true)
            .pretty()
            .with_timer(ChronoLocal::new("%Y-%m-%d %H:%M:%S".to_owned()))
            .with_max_level(level)
            .finish();

        fmt_subscriber
            .with(sentry::integrations::tracing::layer())
            .init();
    }
}
