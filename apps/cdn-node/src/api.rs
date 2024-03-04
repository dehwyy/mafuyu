use std::collections::HashMap;
use axum::{Router, routing, extract::{Query, Path}, http::StatusCode, response::Response};
use axum::response::IntoResponse;

use tower::ServiceBuilder;

use logger as tracing;
use logger::{error, info};

use super::internal::fs::CDNFs;
use super::internal::image::Image;

pub async fn start_api_runtime() {
    let cfg = makoto_config::hosts::Hosts::new();

    let app = Router::new()
        .route("/v1/static/:image", routing::get(serve_image))
        .layer(
            ServiceBuilder::new()
                .layer(mafuyu_sentry::tower::get_sentry_layer())
                .layer(logger::tower::get_tower_tracing_layer())
        );

    info!("Running server on {}", cfg.cdn_api);

    let listener = tokio::net::TcpListener::bind(cfg.cdn_api).await.unwrap();
    axum::serve(listener, app).await.unwrap();

}

/// ### Params:
/// - size (md(medium), sm(small)) - optional

#[tracing::instrument]
async fn serve_image(
    Path(image_name): Path<String>,
    Query(params): Query<HashMap<String, String>>
) -> Result<Vec<u8>, StatusCode> {
    let img = image_name.rsplit_once(".").map(|(filename, ext)| {
        CDNFs::read_image(filename, ext, params.get("size").map(|v| v.clone()))
    });

     match img {
        Some(img_fut) => img_fut.await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR),
        None => Err(StatusCode::BAD_REQUEST)
    }
}