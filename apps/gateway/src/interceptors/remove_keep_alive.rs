use makoto_logger::info;

pub struct RemoveKeepAliveHeaderInterceptor;

impl RemoveKeepAliveHeaderInterceptor {
    pub fn intercept(mut request: tonic::Request<()>, ) -> Result<tonic::Request<()>, tonic::Status> {
        let _ = request.metadata_mut().remove("connection");

        Ok(request)
    }
}