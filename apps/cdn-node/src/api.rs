use std::collections::HashMap;
use axum::{Router, routing, extract::{Query, Path}, http::StatusCode, response::Response};
use axum::response::IntoResponse;

use tracing::instrument;
use tower_http::{trace::TraceLayer};
use tower::ServiceBuilder;

use logger::{error, info};

use super::internal::fs::CDNFs;
use super::internal::image::{PipeImagePayload, Image};

pub async fn start_api_runtime() {
    let cfg = makoto_config::hosts::Hosts::new();

    let app = Router::new()
        .route("/v1/static/:image", routing::get(serve_image))
        .layer(
            ServiceBuilder::new()
                .layer(sentry_tower::NewSentryLayer::new_from_top())
                .layer(TraceLayer::new_for_http())
        );

    info!("Running server on {}", cfg.cdn_api);

    let listener = tokio::net::TcpListener::bind(cfg.cdn_api).await.unwrap();
    axum::serve(listener, app).await.unwrap();

}

/// ### Params:
/// - height || h
/// - width || w
/// - left? (not yet)
/// - top? (not yet)

#[instrument]
async fn serve_image(
    Path(image_name): Path<String>,
    Query(params): Query<HashMap<String, u32>>
) -> Result<impl IntoResponse, StatusCode> {
    let img = image_name.rsplit_once(".").map(|(filename, ext)| {
        CDNFs::read_image(filename, ext)
    });

    let img = match img {
        Some(Ok(img)) => img,
        Some(Err(_)) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        None => return Err(StatusCode::BAD_REQUEST)
    };
    let payload = PipeImagePayload {
        image: img,
        height: params.get("h").or(params.get("height")).map(|v| v.clone()),
        width: params.get("w").or(params.get("width")).map(|v| v.clone()),
    };

    let buf = Image::pipe_image(payload).map_err(|err| {
        error!("Pipe image: {}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Response::builder()
        .header("Content-Type", "image/jpeg")
        .header("Accept-Ranges", "bytes")
        .body(axum::body::Body::from(buf)).map_err(|_| {
            error!("error creating image response");
            StatusCode::INTERNAL_SERVER_ERROR
        })
}