#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignUpRequest {
    #[prost(string, tag = "1")]
    pub username: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub email: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub password: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignInRequest {
    /// either _username_ or _email_
    #[prost(string, tag = "1")]
    pub username: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub email: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub password: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RefreshRequest {
    #[prost(string, tag = "1")]
    pub refresh_token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignInOauthRequest {
    #[prost(string, tag = "1")]
    pub provider: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub code: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignInTokenRequest {
    #[prost(string, tag = "1")]
    pub token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignOutRequest {
    #[prost(string, tag = "1")]
    pub token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OAuth2RedirectUrlRequest {
    #[prost(string, tag = "1")]
    pub provider: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OAuth2RedirectUrlResponse {
    #[prost(string, tag = "1")]
    pub url: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthenticationServiceResponse {
    #[prost(string, tag = "1")]
    pub username: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthenicationServiceResponseWithRefreshToken {
    #[prost(string, tag = "1")]
    pub username: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub token: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub refresh_token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthenticationResponse {
    #[prost(string, tag = "1")]
    pub username: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub user_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfirmMailRequest {
    #[prost(string, tag = "1")]
    pub email: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfirmMailByCodeRequest {
    #[prost(string, tag = "1")]
    pub code: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProceedToUpdatePasswordRequest {
    #[prost(string, tag = "1")]
    pub old_password: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProceedToUpdatePasswordResponse {
    /// TODO: describe exact without any vulnerabilities
    ///
    /// unique generated code (hashed?)
    #[prost(string, tag = "1")]
    pub ucode: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProceedToRecoverPasswordRequest {
    #[prost(string, tag = "1")]
    pub email: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdatePasswordRequest {
    #[prost(string, tag = "1")]
    pub new_password: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubmitNewPasswordByCodeRequest {
    #[prost(string, tag = "1")]
    pub ucode: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub new_password: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IsEmailAvailableRequest {
    #[prost(string, tag = "1")]
    pub email: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IsUsernameAvailableRequest {
    #[prost(string, tag = "1")]
    pub username: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod auth_rpc_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct AuthRpcClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl AuthRpcClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> AuthRpcClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> AuthRpcClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            AuthRpcClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn sign_up(
            &mut self,
            request: impl tonic::IntoRequest<super::SignUpRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AuthenicationServiceResponseWithRefreshToken>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/api_auth.AuthRpc/SignUp");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("api_auth.AuthRpc", "SignUp"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn sign_in(
            &mut self,
            request: impl tonic::IntoRequest<super::SignInRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AuthenicationServiceResponseWithRefreshToken>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/api_auth.AuthRpc/SignIn");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("api_auth.AuthRpc", "SignIn"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn refresh(
            &mut self,
            request: impl tonic::IntoRequest<super::RefreshRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AuthenticationServiceResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/api_auth.AuthRpc/Refresh");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("api_auth.AuthRpc", "Refresh"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_o_auth2_redirect_url(
            &mut self,
            request: impl tonic::IntoRequest<super::OAuth2RedirectUrlRequest>,
        ) -> std::result::Result<
            tonic::Response<super::OAuth2RedirectUrlResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/api_auth.AuthRpc/CreateOAuth2RedirectUrl",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("api_auth.AuthRpc", "CreateOAuth2RedirectUrl"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn sign_in_oauth(
            &mut self,
            request: impl tonic::IntoRequest<super::SignInOauthRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AuthenicationServiceResponseWithRefreshToken>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/api_auth.AuthRpc/SignInOauth",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("api_auth.AuthRpc", "SignInOauth"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn sign_in_token(
            &mut self,
            request: impl tonic::IntoRequest<super::SignInTokenRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AuthenticationServiceResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/api_auth.AuthRpc/SignInToken",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("api_auth.AuthRpc", "SignInToken"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn sign_out(
            &mut self,
            request: impl tonic::IntoRequest<super::SignOutRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::general::BoolStatus>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/api_auth.AuthRpc/SignOut");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("api_auth.AuthRpc", "SignOut"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn confirm_email(
            &mut self,
            request: impl tonic::IntoRequest<super::ConfirmMailRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::general::BoolStatus>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/api_auth.AuthRpc/ConfirmEmail",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("api_auth.AuthRpc", "ConfirmEmail"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_password(
            &mut self,
            request: impl tonic::IntoRequest<super::SubmitNewPasswordByCodeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AuthenticationServiceResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/api_auth.AuthRpc/UpdatePassword",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("api_auth.AuthRpc", "UpdatePassword"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn is_email_available(
            &mut self,
            request: impl tonic::IntoRequest<super::IsEmailAvailableRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::general::BoolStatus>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/api_auth.AuthRpc/IsEmailAvailable",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("api_auth.AuthRpc", "IsEmailAvailable"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn is_username_available(
            &mut self,
            request: impl tonic::IntoRequest<super::IsUsernameAvailableRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::general::BoolStatus>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/api_auth.AuthRpc/IsUsernameAvailable",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("api_auth.AuthRpc", "IsUsernameAvailable"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod auth_rpc_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with AuthRpcServer.
    #[async_trait]
    pub trait AuthRpc: Send + Sync + 'static {
        async fn sign_up(
            &self,
            request: tonic::Request<super::SignUpRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AuthenicationServiceResponseWithRefreshToken>,
            tonic::Status,
        >;
        async fn sign_in(
            &self,
            request: tonic::Request<super::SignInRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AuthenicationServiceResponseWithRefreshToken>,
            tonic::Status,
        >;
        async fn refresh(
            &self,
            request: tonic::Request<super::RefreshRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AuthenticationServiceResponse>,
            tonic::Status,
        >;
        async fn create_o_auth2_redirect_url(
            &self,
            request: tonic::Request<super::OAuth2RedirectUrlRequest>,
        ) -> std::result::Result<
            tonic::Response<super::OAuth2RedirectUrlResponse>,
            tonic::Status,
        >;
        async fn sign_in_oauth(
            &self,
            request: tonic::Request<super::SignInOauthRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AuthenicationServiceResponseWithRefreshToken>,
            tonic::Status,
        >;
        async fn sign_in_token(
            &self,
            request: tonic::Request<super::SignInTokenRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AuthenticationServiceResponse>,
            tonic::Status,
        >;
        async fn sign_out(
            &self,
            request: tonic::Request<super::SignOutRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::general::BoolStatus>,
            tonic::Status,
        >;
        async fn confirm_email(
            &self,
            request: tonic::Request<super::ConfirmMailRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::general::BoolStatus>,
            tonic::Status,
        >;
        async fn update_password(
            &self,
            request: tonic::Request<super::SubmitNewPasswordByCodeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AuthenticationServiceResponse>,
            tonic::Status,
        >;
        async fn is_email_available(
            &self,
            request: tonic::Request<super::IsEmailAvailableRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::general::BoolStatus>,
            tonic::Status,
        >;
        async fn is_username_available(
            &self,
            request: tonic::Request<super::IsUsernameAvailableRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::general::BoolStatus>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct AuthRpcServer<T: AuthRpc> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: AuthRpc> AuthRpcServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for AuthRpcServer<T>
    where
        T: AuthRpc,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/api_auth.AuthRpc/SignUp" => {
                    #[allow(non_camel_case_types)]
                    struct SignUpSvc<T: AuthRpc>(pub Arc<T>);
                    impl<T: AuthRpc> tonic::server::UnaryService<super::SignUpRequest>
                    for SignUpSvc<T> {
                        type Response = super::AuthenicationServiceResponseWithRefreshToken;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SignUpRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AuthRpc>::sign_up(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SignUpSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api_auth.AuthRpc/SignIn" => {
                    #[allow(non_camel_case_types)]
                    struct SignInSvc<T: AuthRpc>(pub Arc<T>);
                    impl<T: AuthRpc> tonic::server::UnaryService<super::SignInRequest>
                    for SignInSvc<T> {
                        type Response = super::AuthenicationServiceResponseWithRefreshToken;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SignInRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AuthRpc>::sign_in(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SignInSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api_auth.AuthRpc/Refresh" => {
                    #[allow(non_camel_case_types)]
                    struct RefreshSvc<T: AuthRpc>(pub Arc<T>);
                    impl<T: AuthRpc> tonic::server::UnaryService<super::RefreshRequest>
                    for RefreshSvc<T> {
                        type Response = super::AuthenticationServiceResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RefreshRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AuthRpc>::refresh(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RefreshSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api_auth.AuthRpc/CreateOAuth2RedirectUrl" => {
                    #[allow(non_camel_case_types)]
                    struct CreateOAuth2RedirectUrlSvc<T: AuthRpc>(pub Arc<T>);
                    impl<
                        T: AuthRpc,
                    > tonic::server::UnaryService<super::OAuth2RedirectUrlRequest>
                    for CreateOAuth2RedirectUrlSvc<T> {
                        type Response = super::OAuth2RedirectUrlResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::OAuth2RedirectUrlRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AuthRpc>::create_o_auth2_redirect_url(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateOAuth2RedirectUrlSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api_auth.AuthRpc/SignInOauth" => {
                    #[allow(non_camel_case_types)]
                    struct SignInOauthSvc<T: AuthRpc>(pub Arc<T>);
                    impl<
                        T: AuthRpc,
                    > tonic::server::UnaryService<super::SignInOauthRequest>
                    for SignInOauthSvc<T> {
                        type Response = super::AuthenicationServiceResponseWithRefreshToken;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SignInOauthRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AuthRpc>::sign_in_oauth(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SignInOauthSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api_auth.AuthRpc/SignInToken" => {
                    #[allow(non_camel_case_types)]
                    struct SignInTokenSvc<T: AuthRpc>(pub Arc<T>);
                    impl<
                        T: AuthRpc,
                    > tonic::server::UnaryService<super::SignInTokenRequest>
                    for SignInTokenSvc<T> {
                        type Response = super::AuthenticationServiceResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SignInTokenRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AuthRpc>::sign_in_token(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SignInTokenSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api_auth.AuthRpc/SignOut" => {
                    #[allow(non_camel_case_types)]
                    struct SignOutSvc<T: AuthRpc>(pub Arc<T>);
                    impl<T: AuthRpc> tonic::server::UnaryService<super::SignOutRequest>
                    for SignOutSvc<T> {
                        type Response = super::super::general::BoolStatus;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SignOutRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AuthRpc>::sign_out(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SignOutSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api_auth.AuthRpc/ConfirmEmail" => {
                    #[allow(non_camel_case_types)]
                    struct ConfirmEmailSvc<T: AuthRpc>(pub Arc<T>);
                    impl<
                        T: AuthRpc,
                    > tonic::server::UnaryService<super::ConfirmMailRequest>
                    for ConfirmEmailSvc<T> {
                        type Response = super::super::general::BoolStatus;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ConfirmMailRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AuthRpc>::confirm_email(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ConfirmEmailSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api_auth.AuthRpc/UpdatePassword" => {
                    #[allow(non_camel_case_types)]
                    struct UpdatePasswordSvc<T: AuthRpc>(pub Arc<T>);
                    impl<
                        T: AuthRpc,
                    > tonic::server::UnaryService<super::SubmitNewPasswordByCodeRequest>
                    for UpdatePasswordSvc<T> {
                        type Response = super::AuthenticationServiceResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::SubmitNewPasswordByCodeRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AuthRpc>::update_password(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdatePasswordSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api_auth.AuthRpc/IsEmailAvailable" => {
                    #[allow(non_camel_case_types)]
                    struct IsEmailAvailableSvc<T: AuthRpc>(pub Arc<T>);
                    impl<
                        T: AuthRpc,
                    > tonic::server::UnaryService<super::IsEmailAvailableRequest>
                    for IsEmailAvailableSvc<T> {
                        type Response = super::super::general::BoolStatus;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::IsEmailAvailableRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AuthRpc>::is_email_available(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = IsEmailAvailableSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api_auth.AuthRpc/IsUsernameAvailable" => {
                    #[allow(non_camel_case_types)]
                    struct IsUsernameAvailableSvc<T: AuthRpc>(pub Arc<T>);
                    impl<
                        T: AuthRpc,
                    > tonic::server::UnaryService<super::IsUsernameAvailableRequest>
                    for IsUsernameAvailableSvc<T> {
                        type Response = super::super::general::BoolStatus;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::IsUsernameAvailableRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AuthRpc>::is_username_available(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = IsUsernameAvailableSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: AuthRpc> Clone for AuthRpcServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: AuthRpc> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: AuthRpc> tonic::server::NamedService for AuthRpcServer<T> {
        const NAME: &'static str = "api_auth.AuthRpc";
    }
}
