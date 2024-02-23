pub trait HandleError<T, R> {
    fn handle(self) -> Result<T, R>;
}

pub trait ResultedOption<T, R> {
    fn unwrap_or_internal(self, msg: &str) -> Result<T, R>;
    fn unwrap_or_not_found(self, msg: &str) -> Result<T, R>;
    fn unwrap_or_unauthorized(self, msg: &str) -> Result<T, R>;
}