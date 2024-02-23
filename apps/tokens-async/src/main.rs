pub mod router ;
pub mod repo;

use async_nats::jetstream::stream::{Config, RetentionPolicy};
use async_nats::jetstream::consumer;
use futures::TryStreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    logger::Logger::new();

    let client = async_nats::connect(makoto_config::constants::nats::TOKENS_SERVER).await?;
    let js = async_nats::jetstream::new(client.clone());

    let stream = js.create_stream(Config {
        name: "tokens".to_string(),
        description: Some("Tokens microservice stream using NATS".to_string()),
        subjects: vec!["tokens.do.>".to_string()],
        max_age: std::time::Duration::from_secs(60),
        retention: RetentionPolicy::WorkQueue,
        ..Default::default()
    }).await?;

    let stream_consumer = stream.create_consumer(consumer::pull::Config {
        durable_name: Some("tokens".to_string()),
        ..Default::default()
    }).await?;


    let db = makoto_db::new().await.expect("cannot open database connection");
    let token_repo = repo::Repo::new(db);

    let router = router::Router::new(token_repo).await;

    let mut messages_stream = stream_consumer.messages().await?;

    while let Ok(Some(message)) = messages_stream.try_next().await {
        router.handle(message).await
    }

    Ok(())
}
