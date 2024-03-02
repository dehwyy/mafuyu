mod nats;
mod api;
mod internal;

use async_nats::jetstream::{consumer, stream::{Config, RetentionPolicy}};
use futures::TryStreamExt;
use logger::{info, Logger};

fn main() {
    let cfg = makoto_config::secrets::Secrets::new();
    Logger::new(cfg.environment);

    let _guard = mafuyu_sentry::Sentry::new(cfg.sentry_dsn);

    info!("Starting Axum API & NATS runtime...");
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let r_nats = start_nats_runtime();
            let r_api = api::start_api_runtime();
            tokio::join!(r_nats, r_api);
        });
}

async fn start_nats_runtime() -> Result<(), Box<dyn std::error::Error + Sync + Send>> {
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


    let router = nats::Router::new().await;

    let mut messages_stream = stream_consumer.messages().await?;

    while let Ok(Some(message)) = messages_stream.try_next().await {
        router.handle(message).await;
    };

    Ok(())
}
