use axum::{
    routing,
    Router
};
use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::RwLock;

struct AppState {
    data: [RwLock<HashMap<String, HashMap<String, Vec<u8>>>>; 256],
}

#[tokio::main]
async fn main() {
    let state: Arc<AppState> = Arc::new(AppState{
        data: [(); 256].map(|_| RwLock::new(HashMap::new())),
    });

    let app = Router::new()
        .route("/api/health", routing::get(|| async { "OK" }))
        .route("/api/version", routing::get(|| async { "0.1.0" }))
        .with_state(state);

    axum::Server::bind(&"0.0.0.0:9000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
