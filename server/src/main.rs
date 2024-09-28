use axum::{
    extract::{Path, State},
    http::{HeaderValue, Method},
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, PgPool};
use tower_http::cors::CorsLayer;

use sqlx::migrate::Migrator;

static MIGRATOR: Migrator = sqlx::migrate!();

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().expect(".env file not detected!");
    let db_url = std::env::var("DATABASE_URL")?;

    let origins: Vec<HeaderValue> = vec!["http://localhost:5173".parse()?];

    let cors = CorsLayer::new()
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
        ])
        .allow_origin(origins);

    let db = PgPoolOptions::new()
        .max_connections(20)
        .connect(&db_url)
        .await?;

    let app = Router::new()
        .route("/", get(root))
        .route("/foo/:id", get(get_thing))
        .route("/foo", get(db_thing))
        .layer(cors)
        .with_state(db);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    println!("Server listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}

#[axum::debug_handler]
async fn root() -> Json<Foo> {
    Json(Foo {
        name: "Test".into(),
        number: 123,
    })
}

#[axum::debug_handler]
async fn get_thing(Path(path): Path<String>, State(db): State<PgPool>) -> Json<Foo> {
    let result = sqlx::query_as::<_, Foo>("SELECT * FROM test WHERE name = $1")
        .bind(path)
        .fetch_one(&db)
        .await
        .unwrap();

    Json(result)
}

#[axum::debug_handler]
async fn db_thing(State(db): State<PgPool>) -> Json<Vec<Foo>> {
    let result = sqlx::query_as::<_, Foo>("SELECT * FROM test")
        .fetch_all(&db)
        .await
        .unwrap_or(vec![]);

    Json(result)
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
struct Foo {
    name: String,
    number: i32,
}
