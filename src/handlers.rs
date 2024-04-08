use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use chrono::{DateTime, Utc};
use futures_util::TryFutureExt;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::utils::Pagination;

pub async fn health_check() -> StatusCode {
    StatusCode::OK
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Quote {
    id: Uuid,
    username: String,
    quote: String,
    inserted_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Quote {
    pub fn new(username: String, quote: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            username,
            quote,
            inserted_at: now,
            updated_at: now,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateQuote {
    username: String,
    quote: String,
}

pub async fn create_quote(
    State(pool): State<PgPool>,
    Json(CreateQuote { username, quote }): Json<CreateQuote>,
) -> Result<(StatusCode, Json<Quote>), StatusCode> {
    let quote = Quote::new(username, quote);
    let res = sqlx::query!(
        r#"
        INSERT INTO quotes (id, username, quote, inserted_at, updated_at)
        VALUES ($1, $2, $3, $4, $5)
        "#,
        quote.id,
        quote.username,
        quote.quote,
        quote.inserted_at,
        quote.updated_at
    )
    .execute(&pool)
    .await;

    match res {
        Ok(_) => Ok((StatusCode::CREATED, Json(quote))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn read_quotes(
    State(pool): State<PgPool>,
    pagination: Option<Query<Pagination>>,
) -> Result<Json<Vec<Quote>>, StatusCode> {
    let Pagination { page, per_page } = pagination.unwrap_or_default().0;
    let page = page.clamp(1, usize::MAX) - 1;
    let per_page = per_page.clamp(1, 100) as i64;
    let offset = page as i64 * per_page;

    let quotes = sqlx::query_as!(
        Quote,
        "SELECT * FROM quotes LIMIT $1 OFFSET $2",
        per_page,
        offset
    )
    .fetch_all(&pool)
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
    .await?;

    Ok(Json(quotes))
}

pub async fn update_quote(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(quote): Json<Quote>,
) -> StatusCode {
    let now = Utc::now();
    let res = sqlx::query!(
        r#"
        UPDATE quotes
        SET username = $1, quote = $2, updated_at = $3
        WHERE id = $4
        "#,
        quote.username,
        quote.quote,
        now,
        id
    )
    .execute(&pool)
    .await
    .map(|res| match res.rows_affected() {
        0 => StatusCode::NOT_FOUND,
        _ => StatusCode::OK,
    });

    match res {
        Ok(status) => status,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

pub async fn delete_quote(State(pool): State<PgPool>, Path(id): Path<Uuid>) -> StatusCode {
    let res = sqlx::query!("DELETE FROM quotes WHERE id = $1", id)
        .execute(&pool)
        .await
        .map(|res| match res.rows_affected() {
            0 => StatusCode::NOT_FOUND,
            _ => StatusCode::OK,
        });

    match res {
        Ok(status) => status,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
