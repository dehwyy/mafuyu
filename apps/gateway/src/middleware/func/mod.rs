pub mod set_auth_cookies;

use std::future::Future;
use std::pin::Pin;
use tonic::body::BoxBody;
use hyper::Response;

pub type MiddlewareFunctionStatic = &'static (dyn MiddlewareFunc + Sync);
pub type MiddlewareFuncResponse = Pin<Box<dyn Future<Output=Response<BoxBody>> + Send>>;

pub trait MiddlewareFunc {
    fn call(&'static self, req: Response<BoxBody>) -> MiddlewareFuncResponse {
        Box::pin(async move {req})
    }
}