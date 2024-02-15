use std::default;
use http::{HeaderMap, HeaderValue};
use tonic::body::BoxBody;
use super::middleware_func::{MiddlewareFunc, MiddlewareFuncResponse};
use super::header_tools::Cookie;

use makoto_grpc::{METADATA_ACCESS_TOKEN_KEY, METADATA_REFRESH_TOKEN_KEY};

#[derive(Clone)]
pub struct SetTokensCookies;

impl SetTokensCookies {
    pub fn new() -> &'static Self {
        &Self
    }

    fn extract_header(headers: &mut HeaderMap, key: &str) -> Option<String> {
        if let Some(Some(v)) = headers.get(key).map(|v| {
            let s = v.to_str();
            match s {
                Ok(s) => {
                    match s.len() {
                        0 => None,
                        _ => Some(s.to_string())
                    }
                },
                Err(_) => return None
            }
        }) {
            return Some(v)
        };

        return None
    }
}

impl MiddlewareFunc for SetTokensCookies {
    fn call(&'static self, mut req: hyper::Response<BoxBody>) -> MiddlewareFuncResponse{
        Box::pin(async move {
            let mut headers = req.headers_mut();
            if let Some(access_token) = Self::extract_header(&mut headers, METADATA_ACCESS_TOKEN_KEY) {
                headers.append(Cookie::header_key(), Cookie{
                    key: METADATA_ACCESS_TOKEN_KEY.to_string(),
                    value: access_token,
                    ..Default::default()
                }.new());
            }

            if let Some(refresh_token) = Self::extract_header(&mut headers, METADATA_REFRESH_TOKEN_KEY) {
                headers.append(Cookie::header_key(), Cookie{
                    key: METADATA_REFRESH_TOKEN_KEY.to_string(),
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