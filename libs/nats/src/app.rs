use std::collections::HashMap;
use std::fmt::Debug;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use async_nats::jetstream::Message;

use logger as tracing;
use logger::{instrument, error};

use crate::errors::HandleError;
use crate::route::RouteResult;
use crate::tools::Tools;

pub fn wrap_route<'a, S, T>(func: fn(Arc<S>, Message) -> T) -> Box<dyn Fn(Arc<S>, Message) -> Pin<Box<dyn Future<Output = RouteResult> + 'a>> + 'a>
    where
        T: Future<Output = RouteResult> + 'a ,
        S: 'a
{
    Box::new(move |service: Arc<S>, message: Message| Box::pin(func(service, message)))
}

type Routes<S> = HashMap<&'static str, Box<dyn Fn(Arc<S>, Message) -> Pin<Box<dyn Future<Output = RouteResult>>>>>;

pub struct App<T> {
    service: Arc<T>,
    routes: Routes<T>
}

impl<T> Debug for App<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NatsApp <[empty!]>")
    }
}

impl<T> App<T> {
    pub fn new(service: T, routes: Routes<T>) -> Self {
        Self {
            service: Arc::new(service),
            routes
        }
    }

    #[instrument]
    pub async fn handle(&self, message: Message) -> Result<(), HandleError> {
        if let Err(err) =  message.ack().await {
            error!("[cannot ack] {err}");
            return Err(HandleError::CannotAck(err.to_string()));
        }

        let subject = Tools::get_subject(&message.subject);

        let route = match self.routes.get(subject.as_str()) {
            Some(route) => route(self.service.clone(), message).await,
            None => {
                error!("[subject not found]");
                return Err(HandleError::SubjectNotFound(subject));
            }
        };

        if let Err(err) = route {
            error!("[router error] {err}");
            return Err(HandleError::RouteError(err));
        };

        Ok(())
    }
}