use std::net::SocketAddr;

use axum::{routing::get, Router};
use tracing::info;

mod io;
mod obsrv;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let _guard = obsrv::initialise()?;

    let app = Router::new().route("/", get(handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

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
