use sentry::ClientInitGuard;

mod log;

pub fn initialise() -> ClientInitGuard {
    log::start_tracing()
}
