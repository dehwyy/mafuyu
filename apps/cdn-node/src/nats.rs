use std::collections::HashMap;
use std::sync::Arc;
use async_nats::jetstream::Message;

use mafuyu_nats::{tools::Tools, route::RouteResult, payload::cdn::{subject, PublishImageRequest}};
use mafuyu_nats::errors::NatsHandleError;
use mafuyu_nats::app::{App, wrap_route};

use crate::internal::image::Image;

pub struct RouterService;

impl RouterService {
    pub fn new() -> Self {
        Self
    }
}

pub struct Router {
    pub handler: App<RouterService>
}

impl Router {
    pub async fn new() -> Self {
        let service = RouterService::new();

        let routes = HashMap::from([
            (subject::PUBLISH_IMAGE, wrap_route(Self::publish_image))
        ]);


        Self {
            handler: App::new(service, routes)
        }
    }

    async fn publish_image(_service: Arc<RouterService>, message: Message) -> RouteResult {
        let payload = Tools::get_payload::<PublishImageRequest>(&message.payload)?;

        Image::save_image(payload.filename, payload.base64_image, payload.image_ext).await.internal_error()?;

        Ok(())
    }
}