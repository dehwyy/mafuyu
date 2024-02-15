use hyper::body::Body;
use tower::{Layer, Service};
use std::{pin::Pin,task::{Context, Poll}};
use std::future::Future;
use std::ops::Deref;
use tonic::body::BoxBody;

pub mod set_tokens_cookies;
pub mod header_tools;
mod middleware_func;
use middleware_func::MiddlewareFunctionStatic;

use makoto_logger::info;

#[derive(Clone)]
pub struct BaseMiddlewareLayer {
    func: MiddlewareFunctionStatic
}

impl BaseMiddlewareLayer {
    pub fn new(func: MiddlewareFunctionStatic) -> Self {
        Self { func }
    }
}

impl<S> Layer<S> for BaseMiddlewareLayer
where S: Service<hyper::Request<Body>>
{
    type Service = BaseMiddleware<S>;

    fn layer(&self, service: S) -> Self::Service {
        BaseMiddleware {
            inner: service ,
            func: self.func
        }
    }
}

#[derive(Clone)]
pub struct BaseMiddleware<S>
where S: Service<hyper::Request<Body>> {
    inner: S,
    func: MiddlewareFunctionStatic
}

impl<S> Service<hyper::Request<Body>> for BaseMiddleware<S>
    where
        S: Service<hyper::Request<Body>, Response = hyper::Response<BoxBody>> + Clone + Send + 'static,
        S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: hyper::Request<Body>) -> Self::Future {
        let clone = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, clone);

        let func = self.func.clone();

        Box::pin(async move {
            // Do extra async work here...
            let mut response: Self::Response = inner.call(req).await?;
            
            let response = func.call(response).await;

            info!("{:?}", response);

            Ok(response)
        })
    }
}