use env_logger::Env;
use news_letter::{config::get_config, startup::run};
use sqlx::PgPool;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

  let config = get_config().expect("Failed to read config");
  let connection_pool = PgPool::connect(&config.database.connection_string())
    .await
    .expect("Failed to connect Postgres");

  let address = format!("0.0.0.0:{}", config.application_port);
  let listener = TcpListener::bind(address)?;

  run(listener, connection_pool)?.await
}
