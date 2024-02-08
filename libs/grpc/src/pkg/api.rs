/// Generated client implementations.
pub mod api_rpc_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct ApiRpcClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ApiRpcClient<tonic::transport::Channel> {
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
    impl<T> ApiRpcClient<T>
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
        ) -> ApiRpcClient<InterceptedService<T, F>>
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
            ApiRpcClient::new(InterceptedService::new(inner, interceptor))
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
        /// Auth
        pub async fn sign_up(
            &mut self,
            request: impl tonic::IntoRequest<super::super::auth::SignUpRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::auth::AuthenticationResponse>,
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
            let path = http::uri::PathAndQuery::from_static("/api.ApiRpc/SignUp");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("api.ApiRpc", "SignUp"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn sign_in(
            &mut self,
            request: impl tonic::IntoRequest<super::super::auth::SignInRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::auth::AuthenticationResponse>,
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
            let path = http::uri::PathAndQuery::from_static("/api.ApiRpc/SignIn");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("api.ApiRpc", "SignIn"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn sign_in_with_token(
            &mut self,
            request: impl tonic::IntoRequest<super::super::auth::SignInWithTokenRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::auth::AuthenticationResponse>,
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
                "/api.ApiRpc/SignInWithToken",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("api.ApiRpc", "SignInWithToken"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn sign_out(
            &mut self,
            request: impl tonic::IntoRequest<super::super::auth::SignOutRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::general::IsOkResponse>,
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
            let path = http::uri::PathAndQuery::from_static("/api.ApiRpc/SignOut");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("api.ApiRpc", "SignOut"));
            self.inner.unary(req, path, codec).await
        }
        /// Passport
        pub async fn update_username(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::passport::UpdateUsernameRequest,
            >,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
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
                "/api.ApiRpc/UpdateUsername",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("api.ApiRpc", "UpdateUsername"));
            self.inner.unary(req, path, codec).await
        }
        /// Tokens
        pub async fn refresh_the_token(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::tokens::RefreshTheTokenRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::general::IsOkResponse>,
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
                "/api.ApiRpc/RefreshTheToken",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("api.ApiRpc", "RefreshTheToken"));
            self.inner.unary(req, path, codec).await
        }
        /// OAuth2
        pub async fn create_o_auth2_redirect_url(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::oauth2::CreateRedirectUrlRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::oauth2::CreateRedirectUrlResponse>,
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
                "/api.ApiRpc/CreateOAuth2RedirectUrl",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("api.ApiRpc", "CreateOAuth2RedirectUrl"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn sign_in_o_auth2(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::oauth2::ExchangeCodeToTokenRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::auth::AuthenticationResponse>,
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
            let path = http::uri::PathAndQuery::from_static("/api.ApiRpc/SignInOAuth2");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("api.ApiRpc", "SignInOAuth2"));
            self.inner.unary(req, path, codec).await
        }
        /// User
        pub async fn edit_user(
            &mut self,
            request: impl tonic::IntoRequest<super::super::user::EditUserRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
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
            let path = http::uri::PathAndQuery::from_static("/api.ApiRpc/EditUser");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("api.ApiRpc", "EditUser"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_user(
            &mut self,
            request: impl tonic::IntoRequest<super::super::user::GetUserRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::user::GetUserResponse>,
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
            let path = http::uri::PathAndQuery::from_static("/api.ApiRpc/GetUser");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("api.ApiRpc", "GetUser"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod api_rpc_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with ApiRpcServer.
    #[async_trait]
    pub trait ApiRpc: Send + Sync + 'static {
        /// Auth
        async fn sign_up(
            &self,
            request: tonic::Request<super::super::auth::SignUpRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::auth::AuthenticationResponse>,
            tonic::Status,
        >;
        async fn sign_in(
            &self,
            request: tonic::Request<super::super::auth::SignInRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::auth::AuthenticationResponse>,
            tonic::Status,
        >;
        async fn sign_in_with_token(
            &self,
            request: tonic::Request<super::super::auth::SignInWithTokenRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::auth::AuthenticationResponse>,
            tonic::Status,
        >;
        async fn sign_out(
            &self,
            request: tonic::Request<super::super::auth::SignOutRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::general::IsOkResponse>,
            tonic::Status,
        >;
        /// Passport
        async fn update_username(
            &self,
            request: tonic::Request<super::super::passport::UpdateUsernameRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /// Tokens
        async fn refresh_the_token(
            &self,
            request: tonic::Request<super::super::tokens::RefreshTheTokenRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::general::IsOkResponse>,
            tonic::Status,
        >;
        /// OAuth2
        async fn create_o_auth2_redirect_url(
            &self,
            request: tonic::Request<super::super::oauth2::CreateRedirectUrlRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::oauth2::CreateRedirectUrlResponse>,
            tonic::Status,
        >;
        async fn sign_in_o_auth2(
            &self,
            request: tonic::Request<super::super::oauth2::ExchangeCodeToTokenRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::auth::AuthenticationResponse>,
            tonic::Status,
        >;
        /// User
        async fn edit_user(
            &self,
            request: tonic::Request<super::super::user::EditUserRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        async fn get_user(
            &self,
            request: tonic::Request<super::super::user::GetUserRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::user::GetUserResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct ApiRpcServer<T: ApiRpc> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ApiRpc> ApiRpcServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ApiRpcServer<T>
    where
        T: ApiRpc,
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
                "/api.ApiRpc/SignUp" => {
                    #[allow(non_camel_case_types)]
                    struct SignUpSvc<T: ApiRpc>(pub Arc<T>);
                    impl<
                        T: ApiRpc,
                    > tonic::server::UnaryService<super::super::auth::SignUpRequest>
                    for SignUpSvc<T> {
                        type Response = super::super::auth::AuthenticationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::auth::SignUpRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ApiRpc>::sign_up(&inner, request).await
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
                "/api.ApiRpc/SignIn" => {
                    #[allow(non_camel_case_types)]
                    struct SignInSvc<T: ApiRpc>(pub Arc<T>);
                    impl<
                        T: ApiRpc,
                    > tonic::server::UnaryService<super::super::auth::SignInRequest>
                    for SignInSvc<T> {
                        type Response = super::super::auth::AuthenticationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::auth::SignInRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ApiRpc>::sign_in(&inner, request).await
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
                "/api.ApiRpc/SignInWithToken" => {
                    #[allow(non_camel_case_types)]
                    struct SignInWithTokenSvc<T: ApiRpc>(pub Arc<T>);
                    impl<
                        T: ApiRpc,
                    > tonic::server::UnaryService<
                        super::super::auth::SignInWithTokenRequest,
                    > for SignInWithTokenSvc<T> {
                        type Response = super::super::auth::AuthenticationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::auth::SignInWithTokenRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ApiRpc>::sign_in_with_token(&inner, request).await
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
                        let method = SignInWithTokenSvc(inner);
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
                "/api.ApiRpc/SignOut" => {
                    #[allow(non_camel_case_types)]
                    struct SignOutSvc<T: ApiRpc>(pub Arc<T>);
                    impl<
                        T: ApiRpc,
                    > tonic::server::UnaryService<super::super::auth::SignOutRequest>
                    for SignOutSvc<T> {
                        type Response = super::super::general::IsOkResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::auth::SignOutRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ApiRpc>::sign_out(&inner, request).await
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
                "/api.ApiRpc/UpdateUsername" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateUsernameSvc<T: ApiRpc>(pub Arc<T>);
                    impl<
                        T: ApiRpc,
                    > tonic::server::UnaryService<
                        super::super::passport::UpdateUsernameRequest,
                    > for UpdateUsernameSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::passport::UpdateUsernameRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ApiRpc>::update_username(&inner, request).await
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
                        let method = UpdateUsernameSvc(inner);
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
                "/api.ApiRpc/RefreshTheToken" => {
                    #[allow(non_camel_case_types)]
                    struct RefreshTheTokenSvc<T: ApiRpc>(pub Arc<T>);
                    impl<
                        T: ApiRpc,
                    > tonic::server::UnaryService<
                        super::super::tokens::RefreshTheTokenRequest,
                    > for RefreshTheTokenSvc<T> {
                        type Response = super::super::general::IsOkResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::tokens::RefreshTheTokenRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ApiRpc>::refresh_the_token(&inner, request).await
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
                        let method = RefreshTheTokenSvc(inner);
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
                "/api.ApiRpc/CreateOAuth2RedirectUrl" => {
                    #[allow(non_camel_case_types)]
                    struct CreateOAuth2RedirectUrlSvc<T: ApiRpc>(pub Arc<T>);
                    impl<
                        T: ApiRpc,
                    > tonic::server::UnaryService<
                        super::super::oauth2::CreateRedirectUrlRequest,
                    > for CreateOAuth2RedirectUrlSvc<T> {
                        type Response = super::super::oauth2::CreateRedirectUrlResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::oauth2::CreateRedirectUrlRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ApiRpc>::create_o_auth2_redirect_url(&inner, request)
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
                "/api.ApiRpc/SignInOAuth2" => {
                    #[allow(non_camel_case_types)]
                    struct SignInOAuth2Svc<T: ApiRpc>(pub Arc<T>);
                    impl<
                        T: ApiRpc,
                    > tonic::server::UnaryService<
                        super::super::oauth2::ExchangeCodeToTokenRequest,
                    > for SignInOAuth2Svc<T> {
                        type Response = super::super::auth::AuthenticationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::oauth2::ExchangeCodeToTokenRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ApiRpc>::sign_in_o_auth2(&inner, request).await
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
                        let method = SignInOAuth2Svc(inner);
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
                "/api.ApiRpc/EditUser" => {
                    #[allow(non_camel_case_types)]
                    struct EditUserSvc<T: ApiRpc>(pub Arc<T>);
                    impl<
                        T: ApiRpc,
                    > tonic::server::UnaryService<super::super::user::EditUserRequest>
                    for EditUserSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::user::EditUserRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ApiRpc>::edit_user(&inner, request).await
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
                        let method = EditUserSvc(inner);
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
                "/api.ApiRpc/GetUser" => {
                    #[allow(non_camel_case_types)]
                    struct GetUserSvc<T: ApiRpc>(pub Arc<T>);
                    impl<
                        T: ApiRpc,
                    > tonic::server::UnaryService<super::super::user::GetUserRequest>
                    for GetUserSvc<T> {
                        type Response = super::super::user::GetUserResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::user::GetUserRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ApiRpc>::get_user(&inner, request).await
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
                        let method = GetUserSvc(inner);
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
    impl<T: ApiRpc> Clone for ApiRpcServer<T> {
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
    impl<T: ApiRpc> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ApiRpc> tonic::server::NamedService for ApiRpcServer<T> {
        const NAME: &'static str = "api.ApiRpc";
    }
}
