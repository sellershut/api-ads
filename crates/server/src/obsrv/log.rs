use std::time::Duration;

use anyhow::Context;
use opentelemetry::{
    sdk::{trace, Resource},
    KeyValue,
};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_semantic_conventions::resource::{
    DEPLOYMENT_ENVIRONMENT, SERVICE_NAME, SERVICE_VERSION,
};
use sentry::{integrations::tracing::EventFilter, ClientOptions, IntoDsn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn start_tracing() -> anyhow::Result<sentry::ClientInitGuard> {
    let dsn =
        api_utils::unwrap_env_variable("SENTRY_DSN").context("[ENV] SENTRY_DSN is missing")?;
    let otlp_collector = api_utils::unwrap_env_variable("OTLP_COLLECTOR")
        .context("[ENV] OTLP_COLLECTOR is missing")?;

    let guard: sentry::ClientInitGuard = sentry::init(ClientOptions {
        dsn: dsn.into_dsn()?,
        ..Default::default()
    });

    let sentry_layer = sentry::integrations::tracing::layer().event_filter(|md| match md.level() {
        &tracing::Level::ERROR | &tracing::Level::WARN => EventFilter::Event,
        _ => EventFilter::Ignore,
    });

    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint(otlp_collector)
                .with_timeout(Duration::from_secs(3)),
        )
        .with_trace_config(trace::config().with_resource(Resource::new([
            KeyValue::new(SERVICE_NAME, env!("CARGO_PKG_NAME")),
            KeyValue::new(SERVICE_VERSION, env!("CARGO_PKG_VERSION")),
            KeyValue::new(DEPLOYMENT_ENVIRONMENT, "develop"),
        ])))
        .install_batch(opentelemetry::runtime::Tokio)?;

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "server_ads=debug,api_db=debug,api_interface=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .with(sentry_layer)
        .with(tracing_opentelemetry::layer().with_tracer(tracer))
        .init();

    tracing::trace!("tracing is live");

    Ok(guard)
}
