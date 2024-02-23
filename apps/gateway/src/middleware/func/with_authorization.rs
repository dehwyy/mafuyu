use std::borrow::BorrowMut;
use super::{MiddlewareFunc, MiddlewareFuncRequest};
use crate::middleware::tools::Headers;

use makoto_grpc::METADATA_ACCESS_TOKEN_KEY;
use makoto_grpc::metadata::{UserRole, METADATA_USER_ID_KEY, METADATA_USER_ROLE_KEY};
use logger::{error, info};

#[derive(Clone)]
pub struct WithAuthorizationMiddleware;

impl WithAuthorizationMiddleware {
    pub fn new() -> &'static Self {
        &Self
    }
}

impl MiddlewareFunc for WithAuthorizationMiddleware {
    fn before_call(&'static self, mut req: hyper::Request<hyper::Body>) -> MiddlewareFuncRequest {
        Box::pin(async move {
            let mut headers = req.headers_mut();
            if let Some(access_token) = Headers::extract_header(headers, METADATA_ACCESS_TOKEN_KEY) {
                if let Ok(auth_client) = makoto_grpc::RpcClients::get_auth_client().await {
                    match auth_client.clone().borrow_mut().sign_in_with_token(makoto_grpc::pkg::auth::SignInWithTokenRequest {
                        token: access_token

                    }).await {
                        Ok(r) => {
                            let (metadata, r, _) =  r.into_parts();
                            if let Ok(user_id) = r.user_id.parse() {
                                headers.insert(METADATA_USER_ID_KEY, user_id);
                            }
                            match metadata.get(METADATA_USER_ROLE_KEY).map(|v| v.to_str().unwrap_or_default().to_string().parse()) {
                                Some(Ok(v)) => headers.insert(METADATA_USER_ROLE_KEY, v),
                                _ => headers.insert(METADATA_USER_ROLE_KEY, UserRole::Unauthorized.to_string().parse().unwrap()),
                            };
                        },
                        Err(err) => {
                            match err.code() {
                                tonic::Code::Unauthenticated => {
                                    if let Ok(unauthorized) = UserRole::Unauthorized.to_string().parse() {
                                        headers.insert(METADATA_USER_ROLE_KEY, unauthorized);
                                    }
                                }
                                _ => {error!("<WithAuthorizationMiddleware> error code: {:?}", err);}
                            }
                        }
                    }
                }
            }

            req
        })
    }
}