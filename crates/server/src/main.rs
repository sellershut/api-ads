use std::net::SocketAddr;

use anyhow::Context;
use axum::{routing::get, Router};
use tracing::info;

mod io;
mod obsrv;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let _guard = obsrv::initialise()?;

    let app = Router::new().route("/", get(handler));

    let port = api_utils::unwrap_env_variable("PORT").context("[ENV] PORT is missing")?;
    let port = port.parse()?;

    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    info!("listening on {addr}");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(io::quit::shutdown_signal())
        .await?;

    Ok(())
}

#[tracing::instrument]
async fn handler() -> &'static str {
    "hello"
}
