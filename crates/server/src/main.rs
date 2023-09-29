mod obsrv;

fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let _guard = obsrv::initialise()?;

    tracing::warn!("Hello, world!");

    Ok(())
}
