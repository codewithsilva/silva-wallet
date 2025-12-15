use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
  pub host:String,
  
  pub port:u16,
  pub env:String,
}

impl AppConfig {
  pub fn from_env() -> Self {
    let host = env::var("APP_HOST").unwrap_or_else(|_| "127.0.0.1".into());
    let port = env::var("APP_PORT").unwrap_or_else(|_| "3000".into())
    .parse().expect("APP_PORT must be a number");

    let env = env::var("APP_ENV").unwrap_or_else(|_| "development".into());
    Self {host, port, env}
  }
}
