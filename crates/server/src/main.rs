use std::net::SocketAddr;

use anyhow::Context;
use api_interface::create_schema;
use async_graphql_axum::GraphQLSubscription;
use axum::{
    http::{header, HeaderValue, Method},
    routing::get,
    Router,
};
use tower_http::cors::CorsLayer;
use tracing::{debug, info};

use crate::routes::{graphql_handler, graphql_playground};

mod io;
mod obsrv;
mod routes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let _guard = obsrv::initialise()?;

    let schema = create_schema().await?;

    let mut app = Router::new()
        .route("/", get(graphql_playground).post(graphql_handler))
        .route_service("/ws", GraphQLSubscription::new(schema.clone()))
        .with_state(schema);

    if let Some(ref frontend_url) = api_utils::unwrap_env_variable("FRONTEND_URL") {
        debug!(allowed_origin = frontend_url, "setting cors layer");
        app = app.layer(
            CorsLayer::new()
                .allow_origin(frontend_url.parse::<HeaderValue>()?)
                .allow_headers([header::CONTENT_TYPE])
                .allow_methods([Method::GET, Method::POST]),
        );
    }

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
