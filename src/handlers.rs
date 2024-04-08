use axum::{extract::State, http::StatusCode, Json};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Debug, Deserialize)]
pub struct CreateQuote {
    user: String,
    quote: String,
}

pub async fn health_check() -> StatusCode {
    StatusCode::OK
}

pub async fn create_quote(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateQuote>,
) -> StatusCode {
    println!("{:?}", payload);
    StatusCode::CREATED
}
