pub mod pkg;

use tonic::transport::Channel;
use pkg::auth::auth_rpc_client::{self, AuthRpcClient};
use pkg::integrations::integrations_rpc_client::{IntegrationsRpcClient, self};
use pkg::oauth2::o_auth2_rpc_client::{OAuth2RpcClient, self};
use pkg::passport::passport_rpc_client::{PassportRpcClient, self};
use pkg::tokens::tokens_rpc_client::{TokensRpcClient, self};

pub const METADATA_ACCESS_TOKEN_KEY: &str = "x-access-token";
pub const METADATA_REFRESH_TOKEN_KEY: &str = "x-refresh-token";

pub struct Tools;

impl Tools {
    pub fn attach_tokens<T>(access_token: String, refresh_token: Option<String>, response: T) -> tonic::Response<T> {
        let mut response = tonic::Response::new(response);

        if let Ok(token) = access_token.parse() {
            response.metadata_mut().insert(METADATA_ACCESS_TOKEN_KEY, token);
        }

        if let Some(token) = refresh_token {
            if let Ok(token) = token.parse() {
                response.metadata_mut().insert(METADATA_REFRESH_TOKEN_KEY, token);
            }
        }

        response
    }
}

pub struct RpcClients<T = Channel, E = tonic::transport::Error>  {
    pub auth_client: Result<AuthRpcClient<T>, E>,
    pub integrations_client: Result<IntegrationsRpcClient<T>, E>,
    pub oauth2_client: Result<OAuth2RpcClient<T>, E>,
    pub passport_client: Result<PassportRpcClient<T>, E>,
    pub tokens_client: Result<TokensRpcClient<T>, E>
}

impl RpcClients {
    pub async fn get_all_client() -> Self {
        let hosts = makoto_config::hosts::Hosts::new();

        let with_http = |url: &str| format!("http://{}", url);

        let auth_client     = auth_rpc_client::AuthRpcClient::connect(with_http(&hosts.auth)).await;
        let integrations_client     = integrations_rpc_client::IntegrationsRpcClient::connect(with_http(&hosts.integrations)).await;
        let oauth2_client = o_auth2_rpc_client::OAuth2RpcClient::connect(with_http(&hosts.oauth2)).await;
        let passport_client = passport_rpc_client::PassportRpcClient::connect(with_http(&hosts.passport)).await;
        let tokens_client =  tokens_rpc_client::TokensRpcClient::connect(with_http(&hosts.tokens)).await;

        Self {
            auth_client,
            integrations_client,
            oauth2_client,
            passport_client,
            tokens_client
        }
    }

}