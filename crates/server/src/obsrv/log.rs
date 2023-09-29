use sentry::{ClientOptions, IntoDsn};
use sentry_tracing::EventFilter;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn start_tracing() -> anyhow::Result<sentry::ClientInitGuard> {
    let dsn = std::env::var("SENTRY_DSN")?;
    let guard: sentry::ClientInitGuard = sentry::init(ClientOptions {
        dsn: dsn.into_dsn()?,
        ..Default::default()
    });

    let sentry_layer = sentry_tracing::layer().event_filter(|md| match md.level() {
        &tracing::Level::ERROR | &tracing::Level::WARN => EventFilter::Event,
        _ => EventFilter::Ignore,
    });

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "server_ads=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .with(sentry_layer)
        .init();

    tracing::trace!("tracing is live");

    Ok(guard)
}
