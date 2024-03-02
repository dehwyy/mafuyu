use async_nats::jetstream::Message;
use logger::error;

use mafuyu_nats::{tools::Tools, route::RouteResult, payload::cdn::{subject, PublishImageRequest}};
use mafuyu_nats::errors::NatsHandleError;

use super::internal::fs::CDNFs;

pub struct Router;

impl Router {
    pub async fn new() -> Self {
        Self
    }

    pub async fn handle(&self, message: Message) {
        if let Err(err) =  message.ack().await {
            error!("[cannot ack] {err}");
            return;
        }

        let subject = match Tools::get_subject(&message.subject) {
            Ok(subject) => subject,
            Err(err) => {
                error!("[subject error]: {err}");
                return;
            }
        };


        let r = match subject.as_str() {
            subject::PUBLISH_IMAGE_PARSED => self.publish_image(message).await,
            _ => {
                error!("[subject not found]");
                return;
            }
        };

        if let Err(err) = r {
            error!("[router error] {err}");
        };

    }

    async fn publish_image(&self, message: Message) -> RouteResult {
        let payload = Tools::get_payload::<PublishImageRequest>(&message.payload)?;

        CDNFs::save_image(&payload.filename, payload.base64_image, payload.image_ext).internal_error()?;

        Ok(())
    }
}