use axum::{
    extract::Extension,
    http::StatusCode,
    routing::{get, post},
    AddExtensionLayer, Router,
};
use envconfig::Envconfig;
use podinfo_rs::{config::Configuration, error::PodinfoError};
use std::{
    net::{IpAddr, SocketAddr},
    sync::{Arc, RwLock},
};

#[tokio::main]
async fn main() -> Result<(), PodinfoError> {
    let config = Configuration::init_from_env()?;

    let app = Router::new().nest("/_z", z_routes());

    axum::Server::bind(&SocketAddr::new(
        IpAddr::V4(config.bind_ip),
        config.bind_port,
    ))
    .serve(app.into_make_service())
    .await
    .unwrap();

    Ok(())
}

fn z_routes() -> Router {
    Router::new()
        .route("/healthz", get(get_healthz))
        .route("/readyz", get(get_readyz))
        .route("/readyz/enable", post(enable_readyz))
        .route("/readyz/disable", post(disable_readyz))
        .layer(AddExtensionLayer::new(SharedReadyzState::default()))
}

async fn get_healthz() -> StatusCode {
    StatusCode::OK
}

async fn get_readyz(Extension(readyz_state): Extension<SharedReadyzState>) -> StatusCode {
    // TODO: is it ok to unwrap here?
    if readyz_state.read().unwrap().enabled {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    }
}

async fn enable_readyz(Extension(readyz_state): Extension<SharedReadyzState>) -> StatusCode {
    // TODO: is it ok to unwrap here?
    readyz_state.write().unwrap().enabled = true;

    StatusCode::OK
}

async fn disable_readyz(Extension(readyz_state): Extension<SharedReadyzState>) -> StatusCode {
    // TODO: is it ok to unwrap here?
    readyz_state.write().unwrap().enabled = false;

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
