use super::errors::{RouteError, NatsHandleError};

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
    pub fn get_subject(initial_subject: &async_nats::Subject) -> String {
        // Let's simplify :)
        initial_subject.to_string()

        // let parts = initial_subject.splitn(3, ".").collect::<Vec<&str>>();

        // let parsed =parts.get(2).map(|v| v.to_string());

        // match parsed {
        //     Some(s) => Ok(s),
        //     None => Err(SubjectError::MalformedSubjectName("[invalid subject path]: @see(src/internal/tools.rs)".to_string()))
        // }
    }

    pub fn get_payload<'a, T>(b: &'a [u8]) -> Result<T, RouteError>
    where T: serde::Deserialize<'a> {
        crate::message::Decoder::decode(b).invalid_argument_error()
    }
}