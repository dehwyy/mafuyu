use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::sync::Arc;
use async_nats::jetstream::Message;
use uuid::Uuid;
use mafuyu_nats::app::{App, wrap_route};
use crate::repo::{GetTokenRecordBy, Repo as TokenRepo};

use makoto_grpc::pkg::tokens::{tokens_rpc_client::TokensRpcClient, ValidateTokenRequest};

use mafuyu_nats::tools::Tools;
use mafuyu_nats::route::RouteResult;
use mafuyu_nats::errors::NatsHandleError;
use mafuyu_nats::payload::tokens::{ClearTokensRequest, subject::CLEAR_TOKENS};

pub struct RouterService<T = tonic::transport::Channel> {
    repo: TokenRepo,
    tokens_clients: TokensRpcClient<T>
}

impl RouterService {
    pub async fn new(repo: TokenRepo) -> Self {

        let clients = makoto_grpc::RpcClients::get_all_client().await;

        Self {
            repo,
            tokens_clients: clients.tokens_client.unwrap()
        }
    }
}

pub struct Router {
    pub handler: App<RouterService>
}

impl Router {
    pub async fn new(service: RouterService) -> Self {
        let routes = HashMap::from([
            (CLEAR_TOKENS, wrap_route(Self::clear_tokens))
        ]);

        Self {
            handler: App::new(service, routes)
        }
    }

    /// ### Removes `expired` access_tokens from db (any provider)
    async fn clear_tokens(service: Arc<RouterService>, message: Message) ->  RouteResult {

        let payload = Tools::get_payload::<ClearTokensRequest>(&message.payload)?;

        let user_id = Uuid::try_parse(&payload.user_id).invalid_argument_error()?;

        let record = service.repo.get_token_record(GetTokenRecordBy::UserId(user_id)).await.internal_error()?;

        let access_tokens = record.access_tokens.clone().unwrap_or_default();
        let mut valid_access_tokens: Vec<String> = vec!();
        for token in access_tokens {

            let response = service.tokens_clients.clone().borrow_mut().validate_token(tonic::Request::new(ValidateTokenRequest {
                access_token: token.clone(),
                provider: record.provider.clone()
            })).await;

            if response.is_ok() {
                valid_access_tokens.push(token)
            };
        }

        service.repo.set_access_tokens(record, valid_access_tokens).await.internal_error()?;

        Ok(())
    }
}