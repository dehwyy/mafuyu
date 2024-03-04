use tower_http::trace::{TraceLayer, HttpMakeClassifier};
pub fn get_tower_tracing_layer() -> TraceLayer<HttpMakeClassifier> {
    TraceLayer::new_for_http()
}