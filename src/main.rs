use axum::{
    routing,
    Router,
    response::{Response, IntoResponse},
    extract::{Path, State, RawBody},
};
use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::RwLock;
use axum::http::status::StatusCode;

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
        .route("/api/post/one/:primary/:secondary", routing::post(api_post_one))
        .route("/api/get/one/:primary/:secondary", routing::get(api_get_one))
        .route("/api/get/all/:primary", routing::get(api_get_all))
        .route("/api/delete/one/:primary/:secondary", routing::delete(api_del_one))
        .route("/api/delete/all/:primary", routing::delete(api_del_all))
        .with_state(state);

    axum::Server::bind(&"0.0.0.0:9000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn api_post_one(State(state): State<Arc<AppState>>, Path(primary): Path<String>, RawBody(body): RawBody ) -> Response {
    (StatusCode::NOT_FOUND).into_response()
}

async fn api_get_one(State(state): State<Arc<AppState>>, Path((primary, secondary)): Path<(String, String)>) -> Response {
    (StatusCode::NOT_FOUND).into_response()
}

async fn api_get_all(State(state): State<Arc<AppState>>, Path(primary): Path<String>) -> Response {
    (StatusCode::NOT_FOUND).into_response()
}

async fn api_del_one(State(state): State<Arc<AppState>>, Path((primary, secondary)): Path<(String, String)>) -> Response {
    (StatusCode::NOT_FOUND).into_response()
}

async fn api_del_all(State(state): State<Arc<AppState>>, Path(primary): Path<String>) -> Response {
    (StatusCode::NOT_FOUND).into_response()
}
