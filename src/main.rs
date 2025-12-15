mod app;
mod routes;
mod config;

use dotenvy::dotenv;

#[tokio::main]
async fn main() {
  dotenv().ok();
  app::run().await;
}
