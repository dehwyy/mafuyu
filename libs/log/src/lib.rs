use flexi_logger::{Logger as BaseLogger, FlexiLoggerError, AdaptiveFormat};
pub use log::{info, log, warn, error};

pub struct Logger;

impl Logger {
    pub fn init() -> Result<(), FlexiLoggerError> {
        BaseLogger::try_with_str("info")?
            .format(flexi_logger::colored_detailed_format)
            .adaptive_format_for_stdout(AdaptiveFormat::Detailed)
            .start()?;

        Ok(())
    }
}
