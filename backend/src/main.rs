use std::{net::SocketAddr, sync::Arc, time::Duration};

use axum:: {
    routing::get, Router,
};

use axum_error::Result;
use sqlx::sqlite::SqlitePool;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() -> Result<()> {
    let _ = dotenv::dotenv();
    let url = std::env::var("DATABASE_URL")?;
    let pool = SqlitePool::connect(&url).await?;

    let app = Router::new().route("/", get(index)).with_state(pool).layer(CorsLayer::very_permissive());
    let address = SocketAddr::from(([0,0,0,0], 8000));
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn index() -> String {
    format!("Hello World!")
}
