use news_letter::{config::get_config, startup::run};
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let config = get_config().expect("Failed to read config");
  let address = format!("0.0.0.0:{}", config.application_port);
  let listener = TcpListener::bind(address)?;

  run(listener)?.await
}
