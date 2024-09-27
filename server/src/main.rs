use axum::{extract::Path, http::Method, routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cors = CorsLayer::new()
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
        ])
        .allow_origin(Any);

    let app = Router::new()
        .route("/", get(root))
        .route("/:id", get(get_thing))
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    println!("Server listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}

#[axum::debug_handler]
async fn root() -> Json<Foo> {
    Json(Foo {
        name: "Test",
        number: 123,
    })
}

#[axum::debug_handler]
async fn get_thing(Path(path): Path<i32>) -> Json<Foo> {
    Json(Foo {
        name: "Test",
        number: path,
    })
}

#[derive(Debug, Deserialize, Serialize)]
struct Foo {
    name: &'static str,
    number: i32,
}
