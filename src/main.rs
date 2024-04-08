mod handlers;
mod utils;

use axum::{
    routing::{delete, get, post, put},
    Router,
};
use sqlx::postgres::PgPoolOptions;
use std::{env, error::Error};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("localhost:{}", port);
    let listener = TcpListener::bind(addr).await?;

    tracing::info!("Listening on: http://{}", listener.local_addr()?);

    let database_url = env::var("DATABASE_URL")?;
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    let app = Router::new()
        .route("/", get(handlers::health_check))
        .route("/quotes", post(handlers::create_quote))
        .route("/quotes", get(handlers::read_quotes))
        .route("/quotes/:id", put(handlers::update_quote))
        .route("/quotes/:id", delete(handlers::delete_quote))
        .with_state(pool)
        .layer(TraceLayer::new_for_http());

    axum::serve(listener, app).await?;
    Ok(())
}
