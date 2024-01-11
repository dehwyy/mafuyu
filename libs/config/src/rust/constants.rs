pub mod nats {
}

pub mod redis {
  pub const AUTH_CACHE: &str = "auth";
  pub const AUTH_URL: &str = "redis://127.0.0.1:6379";
}
