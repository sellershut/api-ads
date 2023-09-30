use std::net::SocketAddr;

use anyhow::Context;
use api_interface::create_schema;
use axum::{routing::get, Extension, Router};
use tracing::info;

use crate::routes::{graphql_handler, graphql_playground};

mod io;
mod obsrv;
mod routes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let _guard = obsrv::initialise()?;

    let schema = create_schema().await?;

    let app = Router::new()
        .route("/", get(graphql_playground).post(graphql_handler))
        .layer(Extension(schema));

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
