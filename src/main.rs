
use std::sync::Arc;
use std::collections::HashMap;

use axum::{
    routing,
    Router,
    response::{Response, IntoResponse},
    extract::{Path, State},
    http::status::StatusCode,
    body::Bytes,
};
use tokio::sync::RwLock;
use blake2::Digest;

/// struct containing all the shared data
struct AppState {
    /// actual HashMap of HashMap storing the data, hidden behind a RwLock
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

/// Service a request to store a single value, given the primary key, secondary key and the data to
/// store (request body)
async fn api_post_one(State(state): State<Arc<AppState>>, Path((primary,secondary)): Path<(String,String)>, body: Bytes ) -> Response {
    let bin = bin_from_string(&primary) as usize;
    {
        let mut rwl = state.data[bin].write().await;
        let pk;
        if let Some(x) = rwl.get_mut(&primary) {
            pk = x;
        } else {
            rwl.insert(primary.clone(), HashMap::new());
            pk = rwl.get_mut(&primary).unwrap();
        }
        pk.insert(secondary, body.to_vec());
    }
    (StatusCode::OK).into_response()
}

/// Service a request to return a single value, given the primary key and the secondary key
async fn api_get_one(State(state): State<Arc<AppState>>, Path((primary, secondary)): Path<(String, String)>) -> Response {
    let bin = bin_from_string(&primary) as usize;
    {
        let rwl = state.data[bin].read().await;
        let Some(pk) = rwl.get(&primary) else {
            return (StatusCode::NOT_FOUND).into_response();
        };
        let Some(data) = pk.get(&secondary) else {
            return (StatusCode::NOT_FOUND).into_response();
        };
        (StatusCode::OK, data.clone()).into_response()
    }
}

/// Service a request to return all the values stored under the primary key
async fn api_get_all(State(state): State<Arc<AppState>>, Path(primary): Path<String>) -> Response {
    let bin = bin_from_string(&primary) as usize;
    {
        let rwl = state.data[bin].read().await;
        let Some(pk) = rwl.get(&primary) else {
            return (StatusCode::NOT_FOUND).into_response();
        };
        return match rmp_serde::to_vec(&pk) {
            Ok(x) => (StatusCode::OK, x).into_response(),
            Err(_) => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
        };
    }
}

/// Service a request to remove a single value, given the primary key and the secondary key
async fn api_del_one(State(state): State<Arc<AppState>>, Path((primary, secondary)): Path<(String, String)>) -> Response {
    let bin = bin_from_string(&primary) as usize;
    {
        let mut rwl = state.data[bin].write().await;
        {
            let Some(pk) = rwl.get_mut(&primary) else {
                return (StatusCode::NOT_FOUND).into_response();
            };
            let Some(_) = pk.remove(&secondary) else {
                return (StatusCode::NOT_FOUND).into_response();
            };
            if !pk.is_empty() {
                return (StatusCode::OK).into_response();
            }
        }
        let _ = rwl.remove(&primary);
    }
    (StatusCode::OK).into_response()
}

/// Service a request to remove all the values stored under the primary key
async fn api_del_all(State(state): State<Arc<AppState>>, Path(primary): Path<String>) -> Response {
    let bin = bin_from_string(&primary) as usize;
    {
        let mut rwl = state.data[bin].write().await;
        {
            let Some(_) = rwl.remove(&primary) else {
                return (StatusCode::NOT_FOUND).into_response();
            };
        }
    }
    (StatusCode::OK).into_response()
}

/// A helper function to decide which (out of 256 possible) bin the primary key falls into - uses blake2 hash and takes the first byte of the result
fn bin_from_string(code: &str) -> u8 {
    let mut hasher = blake2::Blake2b512::new();
    hasher.update(code.as_bytes());
    let result = hasher.finalize();
    result[0]
}
