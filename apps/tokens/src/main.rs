mod internal;

use async_nats::jetstream::stream::{Config, RetentionPolicy};
use async_nats::jetstream::consumer;
use futures::{StreamExt, TryStreamExt};
use internal::{router, AsyncResult, tools::Tools};
use makoto_logger::{error, info};
use crate::internal::repo;

#[tokio::main]
async fn main() -> AsyncResult<()> {
    makoto_logger::Logger::init()?;

    let client = async_nats::connect(makoto_config::constants::nats::TOKENS_SERVER).await?;
    let js = async_nats::jetstream::new(client.clone());

    let db = makoto_db::new().await.expect("cannot open database connection");

    let db_clone = db.clone();
    tokio::spawn(async move {
        let token_repo = repo::Repo::new(db_clone);
        let router = router::core::Router::new(client.clone(), token_repo);
        let mut subscriber = client.subscribe("tokens.call.>").await
            .expect("cannot subscribe");


        while let Some(message) = subscriber.next().await {
            let subject = message.subject.clone();
            let subject = Tools::get_subject(&subject);

            router.handle(&subject, message).await
        }
    });

    // let response = client.request("tokens.call.validate_token", serde_json::to_string(&router::payload::TokenValidationPayload {
    //     access_token: "not_access_token".to_string(),
    // }).unwrap().into()).await.expect("can't");
    // println!("got a response: {:?}", from_utf8(&response.payload).expect("can't"));

    let stream = js.create_stream(Config {
        name: "tokens".to_string(),
        description: Some("Tokens microservice stream using NATS".to_string()),
        subjects: vec!["tokens.async.>".to_string()],
        max_age: std::time::Duration::from_secs(60),
        retention: RetentionPolicy::WorkQueue,
        ..Default::default()
    }).await?;

    let stream_consumer = stream.create_consumer(consumer::pull::Config {
        durable_name: Some("tokens".to_string()),
        ..Default::default()
    }).await?;

    let mut messages_stream = stream_consumer.messages().await?;
    let token_repo = repo::Repo::new(db.clone());
    let router = router::jetstream::Router::new(token_repo);

    while let Ok(Some(message)) = messages_stream.try_next().await {
        let subject = message.subject.clone();
        let subject = Tools::get_subject(&subject);

        router.handle(&subject, message).await
    }

    Ok(())
}
