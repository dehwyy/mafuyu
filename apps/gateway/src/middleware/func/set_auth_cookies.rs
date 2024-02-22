use tonic::body::BoxBody;
use super::{MiddlewareFunc, MiddlewareFuncResponse};
use crate::middleware::tools::{Cookie, Headers};

use makoto_grpc::{COOKIE_ACCESS_TOKEN_KEY, COOKIE_REFRESH_TOKEN_KEY, METADATA_ACCESS_TOKEN_KEY, METADATA_REFRESH_TOKEN_KEY};

#[derive(Clone)]
pub struct SetTokensCookies;

impl SetTokensCookies {
    pub fn new() -> &'static Self {
        &Self
    }
}

impl MiddlewareFunc for SetTokensCookies {
    fn after_call(&'static self, mut req: hyper::Response<BoxBody>) -> MiddlewareFuncResponse{
        Box::pin(async move {
            let mut headers = req.headers_mut();
            if let Some(access_token) = Headers::extract_header(&mut headers, METADATA_ACCESS_TOKEN_KEY) {
                headers.append(Cookie::header_key(), Cookie{
                    key: COOKIE_ACCESS_TOKEN_KEY.to_string(),
                    value: access_token,
                    ..Default::default()
                }.new());
            }

            if let Some(refresh_token) = Headers::extract_header(&mut headers, METADATA_REFRESH_TOKEN_KEY) {
                headers.append(Cookie::header_key(), Cookie{
                    key: COOKIE_REFRESH_TOKEN_KEY.to_string(),
                    value: refresh_token,
                    ..Default::default()
                }.new());
            }

            headers.append(Cookie::header_key(), Cookie{
                key: "Tester".to_string(),
                value: "test".to_string(),
                ..Default::default()
            }.new());

            req
        })
    }
}