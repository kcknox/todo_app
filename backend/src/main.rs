use std::net::SocketAddr;

use axum:: {
    extract::{Path, State}, routing::get, Form, Json, Router
};

use axum_error::Result;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePool;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() -> Result<()> {
    let _ = dotenv::dotenv();
    let url = std::env::var("DATABASE_URL")?;
    let pool = SqlitePool::connect(&url).await?;

    let app = Router::new()
        .route("/", get(list))
        .route("/create", get(create))
        .route("/delete/:id", get(delete))
        .route("/update", get(update))
        .with_state(pool)
        .layer(CorsLayer::very_permissive());
    let address = SocketAddr::from(([0,0,0,0], 8000));
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    axum::serve(listener, app).await?;

    Ok(())
}

#[derive(Deserialize)]
struct NewTodo {
    description: String,
}

#[derive(Serialize, Deserialize)]
struct Todo {
    id: i64,
    description: String,
    done: bool,
}

async fn list(State(pool): State<SqlitePool>) -> Result<Json<Vec<Todo>>, axum::http::StatusCode> {
    // List all notes
    let todos = sqlx::query_as!(Todo, "SELECT id, description, done FROM todos ORDER BY id")
        .fetch_all(&pool)
        .await.unwrap();

    Ok(Json(todos))
}

async fn create(State(pool): State<SqlitePool>, Form(_todo): Form<NewTodo>) -> Result<String, axum::http::StatusCode> {
    sqlx::query!("INSERT INTO todos (description) VALUES (?)", _todo.description)
        .execute(&pool).await.unwrap();

    Ok(format!("Successfully inserted todo!"))
}

async fn delete(State(pool): State<SqlitePool>, Path(id): Path<i64>) -> Result<String, axum::http::StatusCode> {
    // List all notes
    sqlx::query!("DELETE FROM todos where id = ?", id)
        .execute(&pool).await.unwrap();

    Ok(format!("Successfully deleted todo!"))
}

async fn update(State(pool): State<SqlitePool>, Form(_todo): Form<Todo>) -> Result<String, axum::http::StatusCode> {
    sqlx::query!("UPDATE todos SET description = ?, done = ? WHERE id = ?", _todo.description, _todo.done, _todo.id)
        .execute(&pool).await.unwrap();

    Ok(format!("Successfully updated todo!"))
}