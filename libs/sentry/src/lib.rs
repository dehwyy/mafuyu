pub struct Sentry;

impl Sentry {
    pub fn new(dsn: String) -> sentry::ClientInitGuard {
        let sentry = sentry::init((dsn, sentry::ClientOptions{
            release: sentry::release_name!(),
            traces_sample_rate: 1.0,
            sample_rate: 1.0,
            debug: true,
            ..Default::default()
        }));

        sentry
    }
}
