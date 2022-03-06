use axum::{
    extract::Extension,
    http::StatusCode,
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tokio::sync::RwLock;

pub fn z_routes() -> Router {
    Router::new()
        .route("/healthz", get(get_healthz))
        .route("/readyz", get(get_readyz))
        .route("/readyz/enable", post(enable_readyz))
        .route("/readyz/disable", post(disable_readyz))
        .layer(Extension(SharedReadyzState::default()))
}

async fn get_healthz() -> StatusCode {
    StatusCode::OK
}

async fn get_readyz(Extension(readyz_state): Extension<SharedReadyzState>) -> StatusCode {
    if readyz_state.read().await.enabled {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    }
}

async fn enable_readyz(Extension(readyz_state): Extension<SharedReadyzState>) -> StatusCode {
    readyz_state.write().await.enabled = true;

    StatusCode::OK
}

async fn disable_readyz(Extension(readyz_state): Extension<SharedReadyzState>) -> StatusCode {
    readyz_state.write().await.enabled = false;

    StatusCode::OK
}

type SharedReadyzState = Arc<RwLock<ReadyzState>>;

struct ReadyzState {
    enabled: bool,
}

impl Default for ReadyzState {
    fn default() -> Self {
        Self { enabled: true }
    }
}
