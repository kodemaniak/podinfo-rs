use axum::{
    extract::Extension,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use build_info::BuildInfo;
use std::sync::Arc;
use tokio::sync::RwLock;

build_info::build_info!(fn build_info);

pub fn z_routes() -> Router {
    Router::new()
        .route("/healthz", get(get_healthz))
        .route("/readyz", get(get_readyz))
        .route("/readyz/enable", post(enable_readyz))
        .route("/readyz/disable", post(disable_readyz))
        .route("/buildz", get(get_buildz))
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

async fn get_buildz() -> Json<BuildInfo> {
    Json(build_info().to_owned())
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
