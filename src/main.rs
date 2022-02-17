use axum::routing::post;
use axum::Router;
use envconfig::Envconfig;
use log::info;
use podinfo_rs::echo::echo;
use podinfo_rs::z;
use podinfo_rs::{config::Configuration, error::PodinfoError};
use std::net::{IpAddr, SocketAddr};
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() -> Result<(), PodinfoError> {
    tracing_subscriber::fmt::init();

    let config = Configuration::init_from_env()?;

    let app = Router::new()
        .nest("/_z", z::z_routes().layer(TraceLayer::new_for_http()))
        .route("/echo", post(echo))
        .layer(TraceLayer::new_for_http());

    info!("Starting podinfo-rs...");
    axum::Server::bind(&SocketAddr::new(
        IpAddr::V4(config.bind_ip),
        config.bind_port,
    ))
    .serve(app.into_make_service())
    .await
    .unwrap();

    Ok(())
}
