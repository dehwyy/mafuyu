pub mod set_auth_cookies;
pub mod with_authorization;

use std::future::Future;
use std::pin::Pin;
use tonic::body::BoxBody;
use hyper::{Response, Request, Body};

pub type MiddlewareFunctionStatic = &'static (dyn MiddlewareFunc + Sync);
pub type MiddlewareFuncRequest = Pin<Box<dyn Future<Output=Request<Body>> + Send>>;
pub type MiddlewareFuncResponse = Pin<Box<dyn Future<Output=Response<BoxBody>> + Send>>;

pub trait MiddlewareFunc {
    fn before_call(&'static self, mut req: Request<Body>) -> MiddlewareFuncRequest {
        Box::pin(async move {req})
    }
    fn after_call(&'static self, mut res: Response<BoxBody>) -> MiddlewareFuncResponse {
        Box::pin(async move {res})
    }
}