use sentry::ClientInitGuard;

mod log;

pub fn initialise() -> anyhow::Result<ClientInitGuard> {
    log::start_tracing()
}
