use std::borrow::BorrowMut;
use async_nats::jetstream::Message;
use uuid::Uuid;
use mafuyu_nats::message::MessageError;
use makoto_lib::errors::prelude::*;
use makoto_logger::error;
use crate::repo::{GetTokenRecordBy, Repo as TokenRepo};

use makoto_grpc::pkg::{tokens::{tokens_rpc_client::TokensRpcClient, ValidateTokenRequest}, integrations::{integrations_rpc_client::IntegrationsRpcClient, GetBasicUserRequest}};

use mafuyu_nats::tools::Tools;
use mafuyu_nats::route::{RouteError, RouteResult};
use mafuyu_nats::payload::tokens::ClearTokensRequest;

pub struct Router<T = tonic::transport::Channel> {
    repo: TokenRepo,
    integrations_client: IntegrationsRpcClient<T>,
    tokens_clients: TokensRpcClient<T>
}

impl Router {
    pub async fn new(token_repo: TokenRepo) -> Self {

        let clients = makoto_grpc::RpcClients::get_all_client().await;

        Self {
            repo: token_repo,
            integrations_client: clients.integrations_client.unwrap(),
            tokens_clients: clients.tokens_client.unwrap()
        }
    }

    pub async fn handle(&self, message: Message) {
        if let Err(err) =  message.ack().await {
            error!("[cannot ack] {err}");
            return;
        }

        let subject = match Tools::get_subject(&message.subject) {
            Ok(subject) => subject,
            Err(err) => {
                error!("[subject error]: {err}");
                return;
            }
        };


        let r = match subject.as_str() {
            "clear_tokens" => self.clear_tokens(message).await,
            _ => {
                error!("[subject not found]");
                return;
            }
        };

        if let Err(err) = r {
            error!("[router error] {err}");
        };

    }

    /// ### Removes `expired` access_tokens from db (any provider)
    async fn clear_tokens(&self, message: Message) ->  RouteResult {

        let payload = Tools::get_payload::<ClearTokensRequest>(&message.payload)?;

        let user_id = Uuid::try_parse(&payload.user_id).map_err(|err| {
            RouteError::MessageError(MessageError::MalformedRequest(err.to_string()))
        })?;

        let record = self.repo.get_token_record(GetTokenRecordBy::UserId(user_id)).await.handle_nats()?;

        let access_tokens = record.access_tokens.clone().unwrap_or_default();
        let mut valid_access_tokens: Vec<String> = vec!();
        for token in access_tokens {

            let is_valid = match &record.provider {
                Some(oauth2_provider) => {
                    // Fetch user. If `succeed -> user was fetch from `provider` with current token
                    self.integrations_client.clone().borrow_mut().get_basic_user(tonic::Request::new(GetBasicUserRequest {
                        provider: oauth2_provider.to_string(),
                        access_token: token.clone()
                    })).await.is_ok()
                },
                None => {
                    self.tokens_clients.clone().borrow_mut().validate_token(tonic::Request::new(ValidateTokenRequest {
                        access_token: token.clone()
                    })).await.is_ok()
                }
            };

            if is_valid {
                valid_access_tokens.push(token)
            };
        }

        self.repo.set_access_tokens(record, valid_access_tokens).await.handle_nats()?;

        Ok(())
    }
}