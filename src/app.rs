use axum::{Router};
use crate::{routes, config::AppConfig};

pub async fn run() {
  let config = AppConfig::from_env();
  let app = Router::new().merge(routes::router());

  let addr = format!("{}:{}", config.host, config.port);
  let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
  .await.expect("Failed to Bind Address");

  println!("Silva Bank http://{} ({})",addr, config.env);
  axum::serve(listener, app).await.unwrap();
}
