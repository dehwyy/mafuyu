mod router ;
mod fs;

use async_nats::jetstream::{consumer, stream::{Config, RetentionPolicy}};
use futures::TryStreamExt;
use logger::Logger;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let cfg = makoto_config::secrets::Secrets::new();
    Logger::new(cfg.environment);

    let client = async_nats::connect(makoto_config::constants::nats::CDN_SERVER).await?;
    let js = async_nats::jetstream::new(client.clone());

    let stream = js.create_stream(Config {
        name: "cdn".to_string(),
        description: Some("Self-hosted CDN :0".to_string()),
        subjects: vec!["cdn.do.>".to_string()],
        max_age: std::time::Duration::from_secs(60),
        retention: RetentionPolicy::Interest, // all consumers should be acked
        ..Default::default()
    }).await?;

    let stream_consumer = stream.create_consumer(consumer::pull::Config {
        durable_name: Some("cdn".to_string()),
        ..Default::default()
    }).await?;


    let cdn_fs = fs::CDNFs::new();
    let router = router::Router::new(cdn_fs).await;

    let mut messages_stream = stream_consumer.messages().await?;

    while let Ok(Some(message)) = messages_stream.try_next().await {
        router.handle(message).await
    }

    Ok(())
}
