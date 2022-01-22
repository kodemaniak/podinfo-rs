use axum::routing::post;
use axum::Router;
use envconfig::Envconfig;
use podinfo_rs::echo::echo;
use podinfo_rs::z;
use podinfo_rs::{config::Configuration, error::PodinfoError};
use std::net::{IpAddr, SocketAddr};

#[tokio::main]
async fn main() -> Result<(), PodinfoError> {
    let config = Configuration::init_from_env()?;

    let app = Router::new()
        .nest("/_z", z::z_routes())
        .route("/echo", post(echo));

    axum::Server::bind(&SocketAddr::new(
        IpAddr::V4(config.bind_ip),
        config.bind_port,
    ))
    .serve(app.into_make_service())
    .await
    .unwrap();

    Ok(())
}
