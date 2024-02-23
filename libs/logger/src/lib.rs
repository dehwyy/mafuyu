use tracing_subscriber::fmt::time::ChronoLocal;
pub use tracing::{trace, debug, info, error, warn};

pub struct Logger;

impl Logger {
    pub fn new() {
        let subscriber = tracing_subscriber::FmtSubscriber::builder()
            .pretty()
            .with_target(true)
            .with_file(true)
            .with_line_number(true)
            .with_ansi(true)
            .with_timer(ChronoLocal::new("%Y-%m-%d %H:%M:%S".to_owned()))
            .finish();

        tracing::subscriber::set_global_default(subscriber).unwrap();
    }
}
