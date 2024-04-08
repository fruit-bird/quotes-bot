mod handlers;

use axum::{
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use std::{env, error::Error};
use tokio::net::TcpListener;
use uuid::Uuid;

use handlers::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Quote {
    id: Uuid,
    user: String,
    quote: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(addr).await?;

    let database_url = env::var("DATABASE_URL")?;
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    let app = Router::new()
        .route("/", get(health_check))
        .route("/quotes", post(create_quote))
        .with_state(pool);

    axum::serve(listener, app).await?;
    Ok(())
}
