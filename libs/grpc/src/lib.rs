pub mod pkg;
pub mod errors;
pub mod metadata;

use tonic::transport::Channel;
use pkg::auth::auth_rpc_client::{self, AuthRpcClient};
use pkg::authorization::authorization_rpc_client;
use pkg::integrations::integrations_rpc_client::{IntegrationsRpcClient, self};
use pkg::oauth2::o_auth2_rpc_client::{OAuth2RpcClient, self};
use pkg::passport::passport_rpc_client::{PassportRpcClient, self};
use pkg::tokens::tokens_rpc_client::{TokensRpcClient, self};
use pkg::user::user_rpc_client::{UserRpcClient, self};
use pkg::cdn::cdn_rpc_client::{CdnRpcClient, self};
use pkg::mailer::mailer_rpc_client;

pub const METADATA_ACCESS_TOKEN_KEY: &str = "x-access-token";
pub const COOKIE_ACCESS_TOKEN_KEY: &str = "access_token";
pub const METADATA_REFRESH_TOKEN_KEY: &str = "x-refresh-token";
pub const COOKIE_REFRESH_TOKEN_KEY: &str = "refresh_token";

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
    pub authorization_client: Result<authorization_rpc_client::AuthorizationRpcClient<T>, E>,
    pub integrations_client: Result<IntegrationsRpcClient<T>, E>,
    pub oauth2_client: Result<OAuth2RpcClient<T>, E>,
    pub passport_client: Result<PassportRpcClient<T>, E>,
    pub tokens_client: Result<TokensRpcClient<T>, E>,
    pub user_client: Result<UserRpcClient<T>, E>,
    pub cdn_client: Result<CdnRpcClient<T>, E>,
    pub mailer_client: Result<mailer_rpc_client::MailerRpcClient<T>, E>,
}

impl RpcClients {
    pub async fn get_auth_client() -> Result<AuthRpcClient<Channel>, tonic::transport::Error> {
        let url = format!("http://{}", makoto_config::hosts::Hosts::new().auth);
        auth_rpc_client::AuthRpcClient::connect(url).await
    }

    pub async fn get_all_client() -> Self {
        let hosts = makoto_config::hosts::Hosts::new();

        let with_http = |url: &str| format!("http://{}", url);

        Self {
            auth_client:                   auth_rpc_client::AuthRpcClient::connect(with_http(&hosts.auth)).await,
            authorization_client: authorization_rpc_client::AuthorizationRpcClient::connect(with_http(&hosts.authorization)).await,
            integrations_client:   integrations_rpc_client::IntegrationsRpcClient::connect(with_http(&hosts.integrations)).await,
            oauth2_client:              o_auth2_rpc_client::OAuth2RpcClient::connect(with_http(&hosts.oauth2)).await,
            passport_client:           passport_rpc_client::PassportRpcClient::connect(with_http(&hosts.passport)).await,
            tokens_client:               tokens_rpc_client::TokensRpcClient::connect(with_http(&hosts.tokens)).await,
            user_client:                   user_rpc_client::UserRpcClient::connect(with_http(&hosts.user)).await,
            cdn_client:                     cdn_rpc_client::CdnRpcClient::connect(with_http(&hosts.cdn_rpc)).await,
            mailer_client:               mailer_rpc_client::MailerRpcClient::connect(with_http(&hosts.mailer_rpc)).await
        }
    }
}