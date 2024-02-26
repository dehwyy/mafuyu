use tracing_subscriber::fmt::time::ChronoLocal;
pub use tracing::{trace, debug, info, error, warn};

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

impl Into<tracing::Level> for LoggerMode {
    fn into(self) -> tracing::Level {
        match self {
            LoggerMode::Trace => tracing::Level::TRACE,
            LoggerMode::Default => tracing::Level::INFO,
            LoggerMode::Production => tracing::Level::ERROR
        }
    }
}

pub struct Logger;

impl Logger {
    pub fn new(environment: String) {
        let mode = LoggerMode::new(environment);
        let level: tracing::Level = mode.clone().into();

        let base = tracing_subscriber::FmtSubscriber::builder()
            .with_target(true)
            .with_file(true)
            .with_line_number(true)
            .with_ansi(true)
            .with_timer(ChronoLocal::new("%Y-%m-%d %H:%M:%S".to_owned()))
            .with_max_level(level);

        match mode {
            LoggerMode::Production => tracing::subscriber::set_global_default(base.compact().finish()).unwrap(),
            _ => tracing::subscriber::set_global_default(base.pretty().finish()).unwrap()
        }
    }
}
