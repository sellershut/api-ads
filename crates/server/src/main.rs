mod obsrv;

fn main() {
    let _guard = obsrv::initialise();
    tracing::warn!("Hello, world!");
}
