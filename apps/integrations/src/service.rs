use tonic::{Request, Response, Status};
use makoto_grpc::pkg::integrations as rpc;
use makoto_grpc::pkg::integrations::integrations_rpc_server::IntegrationsRpc;
use makoto_lib::errors::SafeUnwrapWithMessage;

use crate::provider::{Provider, ProviderTrait};

pub struct IntegrationRpcServiceImplementation {

}

impl IntegrationRpcServiceImplementation {
    pub fn new() -> Self {
        Self {}
    }
}

#[tonic::async_trait]
impl IntegrationsRpc for IntegrationRpcServiceImplementation {
    async fn get_basic_user(&self, req: Request<rpc::GetBasicUserRequest>) -> Result<Response<rpc::GetBasicUserResponse>, Status> {
        let req = req.into_inner();

        let provider = Provider::from_str(&req.provider).safe_unwrap("requested provider is not supported")?.as_provider();
        let response =  provider.get_basic_user(req.access_token).await.map_err(|err| err.as_tonic_status())?;

        Ok(Response::new(response))

    }
}