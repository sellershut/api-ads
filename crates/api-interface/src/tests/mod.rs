use crate::create_schema;

#[tokio::test]
async fn query() -> Result<(), Box<dyn std::error::Error>> {
    let schema = create_schema().await?;

    schema.execute("").await;

    Ok(())
}
