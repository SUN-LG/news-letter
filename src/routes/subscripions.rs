use actix_web::{
  HttpResponse,
  web::{Data, Form},
};
use sqlx::PgPool;

#[derive(serde::Deserialize)]
pub struct FormData {
  email: String,
  name: String,
}

pub async fn subscribe(form: Form<FormData>, pool: Data<PgPool>) -> HttpResponse {
  match sqlx::query!(
    r#"
      insert into subscriptions (email, name, subscribe_at)
      values ($1, $2, $3)
    "#,
    form.email,
    form.name,
    chrono::Utc::now()
  )
  .execute(pool.get_ref())
  .await
  {
    Ok(_) => HttpResponse::Ok().finish(),
    Err(e) => {
      println!("Failed to execute query: {e}");
      HttpResponse::InternalServerError().finish()
    }
  }
}
