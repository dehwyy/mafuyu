use async_nats::Subject;

pub struct Tools;

impl Tools {
    /// # Subject Convention.
    /// - Starts with `tokens`.
    /// - Second path's part either `call` or `do`.
    /// - At least `3` path's parts
    /// ## Description.
    /// - `call` means `request-reply` scenario
    /// - `do` means `act as jetstream` (async communication, w/o response).
    /// Example of valid `path` is `tokens.call.validate_token` or `tokens.do.clear.all`
    pub fn get_subject(initial_subject: &Subject) -> String {
        let parts = initial_subject.splitn(3, ".").collect::<Vec<&str>>();

        parts
            .get(2).map(|v| v.to_string())
            .unwrap_or("$(invalid path, @see(src/internal/tools.rs)".to_string())
    }
}