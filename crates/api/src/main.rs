use axum::{routing::get, Json, Router};
use tower_http::cors::{Any, CorsLayer};
use visualizer_types::HealthResponse;

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
    })
}

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/health", get(health))
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("API server running on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
