use async_nats::jetstream::Message;
use makoto_logger::error;

use mafuyu_nats::{tools::Tools, route::RouteResult, payload::cdn::{subject, PublishImageRequest}};
use mafuyu_nats::errors::NatsHandleError;

pub struct Router {
    cdn_fs: crate::fs::CDNFs
}

impl Router {
    pub async fn new(cdn_fs: crate::fs::CDNFs) -> Self {

        Self {
            cdn_fs
        }
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

    pub async fn publish_image(&self, message: Message) -> RouteResult {
        let payload = Tools::get_payload::<PublishImageRequest>(&message.payload)?;


        self.cdn_fs.save_image(&payload.filename, payload.base64_image, payload.image_ext).internal_error()?;

        Ok(())
    }
}