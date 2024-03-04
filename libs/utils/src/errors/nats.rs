#[cfg(feature = "rpc")]
pub mod with_rpc {
    use tonic::Status;
    use mafuyu_nats::errors::MessageError;
    use crate::errors::prelude::HandleError;

    impl<T> HandleError<T, Status> for Result<T, MessageError> {
        fn handle(self) -> Result<T, Status> {
            self.map_err(|err| {
                match err {
                    MessageError::MalformedRequest(err) => Status::invalid_argument(err),
                    MessageError::CannotDeserialize(err) => Status::internal(err.to_string()),
                    _ => Status::internal(err.to_string())
                }
            })
        }
    }

    impl<T> HandleError<T, Status> for Result<T, async_nats::PublishError> {
        fn handle(self) -> Result<T, Status> {
            self.map_err(|err| Status::internal(err.to_string()))
        }
    }
}

