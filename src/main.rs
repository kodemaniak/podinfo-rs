use std::net::{IpAddr, SocketAddr};

use axum::{http::StatusCode, routing::get, Router};
use envconfig::Envconfig;
use podinfo_rs::config::Configuration;

#[tokio::main]
async fn main() -> Result<(), PodinfoError> {
    let config = Configuration::init_from_env()?;

    let app = Router::new()
        .route("/healthz", get(|| async { StatusCode::OK }))
        .route("/readyz", get(|| async { StatusCode::OK }));

    axum::Server::bind(&SocketAddr::new(
        IpAddr::V4(config.bind_ip),
        config.bind_port,
    ))
    .serve(app.into_make_service())
    .await
    .unwrap();

    Ok(())
}

#[derive(Debug)]
enum PodinfoError {
    ConfigationError(String),
}

impl From<envconfig::Error> for PodinfoError {
    fn from(e: envconfig::Error) -> Self {
        match e {
            envconfig::Error::ParseError { name } => {
                PodinfoError::ConfigationError(format!("Failed to parse environment var {}!", name))
            }
            envconfig::Error::EnvVarMissing { name } => {
                PodinfoError::ConfigationError(format!("Missing environment var: {}", name))
            }
        }
    }
}
