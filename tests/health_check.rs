//! tests/health_check.rs
//  `actix_web::test` is the testing equivalent of `actix_web::main`.
//  It also spares you from having to specify the `#[test]` attribute.
//
//  You can inspect what code gets generated using
//  `cargo expand --test health_check` (<- name of the test file)

use std::net::TcpListener;

use news_letter::config::get_config;
use sqlx::{Connection, PgConnection, PgPool};

#[actix_web::test]
async fn health_check_works() {
  // Arrange
  let app = spawn_app().await;

  let client = reqwest::Client::new();
  // Act
  let response = client
    .get(format!("{}/health_check", app.address.as_str()))
    .send()
    .await
    .expect("Failed to execute request.");

  // Assert
  assert!(response.status().is_success());
  assert_eq!(Some(0), response.content_length());
}

pub struct TestApp {
  pub address: String,
  pub db_pool: PgPool,
}

async fn spawn_app() -> TestApp {
  let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
  let port = listener.local_addr().unwrap().port();
  let address = format!("http://127.0.0.1:{}", port);

  let config = get_config().expect("Failed to read config");
  let connection_pool = PgPool::connect(&config.database.connection_string())
    .await
    .expect("Failed to connect to Postgres");

  let server =
    news_letter::startup::run(listener, connection_pool.clone()).expect("Failed to spawn our app.");
  let _ = actix_web::rt::spawn(server);

  TestApp {
    address,
    db_pool: connection_pool,
  }
}

#[actix_web::test]
async fn subscribe_returns_200_for_valid_form_data() {
  // Arrange
  let app = spawn_app().await;
  let app_address = app.address.as_str();

  let client = reqwest::Client::new();

  // Act
  let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
  let response = client
    .post(format!("{}/subscriptions", app_address))
    .header("Content-Type", "application/x-www-form-urlencoded")
    .body(body)
    .send()
    .await
    .expect("Failed to execute request.");

  // Assert
  assert_eq!(200, response.status().as_u16());

  let saved = sqlx::query!("select email, name from subscriptions")
    .fetch_one(&app.db_pool)
    .await
    .expect("Failed to fetch saved subscription.");

  assert_eq!(saved.email, "ursula_le_guin@gmail.com");
  assert_eq!(saved.name, "le guin");
}

#[actix_web::test]
async fn subscribe_returns_400_when_data_is_missing() {
  // Arrange
  let app = spawn_app().await;
  let client = reqwest::Client::new();
  let test_cases = vec![
    ("name=le%20guin", "missing the email"),
    ("email=ursula_le_guin%40gmail.com", "missing the name"),
    ("", "missing both name and email"),
  ];

  // Act
  for (invalid_body, error_message) in test_cases {
    let response = client
      .post(format!("{}/subscriptions", app.address.as_str()))
      .header("Content-Type", "application/x-www-form-urlencoded")
      .body(invalid_body)
      .send()
      .await
      .expect("Failed to execute request.");

    // Assert
    assert_eq!(
      400,
      response.status().as_u16(),
      "The api did not fail with 400 bad request when the payload was {}.",
      error_message
    )
  }
}
