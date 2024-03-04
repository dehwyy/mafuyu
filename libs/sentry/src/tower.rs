pub fn get_sentry_layer<T>() -> sentry_tower::NewSentryLayer<T> {
    sentry_tower::NewSentryLayer::new_from_top()
}