use api_core::{
    category::Category,
    engine::{mutation::category::Mutation, query::category::Query},
};

use crate::DatabaseConnection;

async fn db_connect() -> Result<DatabaseConnection, crate::errors::DatabaseError> {
    dotenvy::dotenv().ok();
    DatabaseConnection::from_env().await
}

#[tokio::test]
async fn create_connection() {
    let conn = db_connect().await;
    assert!(conn.is_ok());
}

#[tokio::test]
async fn mutation() -> Result<(), Box<dyn std::error::Error>> {
    let conn = db_connect().await?;

    let mut category = Category::new();

    let name = "Foo";

    category.name = name.to_owned();
    category.is_root = true;

    let mutation_result = conn.create_category(category).await?;

    let id = &mutation_result.id;

    assert_eq!(&mutation_result.name, name);
    assert!(mutation_result.sub_categories.is_empty());
    assert!(mutation_result.is_root);
    assert_eq!(mutation_result.image_url, None);

    let mut category = Category::new();

    let name = "Bar";
    category.name = name.to_string();
    category.is_root = true;

    let update_result = conn.update_category(id, category).await?;

    assert_eq!(&update_result.id, id);
    assert_eq!(&update_result.name, name);
    assert!(mutation_result.is_root);

    let query = conn.get_category_by_id(id).await?.unwrap();

    assert_eq!(&query.id, id);
    assert_eq!(&query.name, name);

    let query = conn.get_categories().await?;

    assert_eq!(query.count(), 1);

    let query = conn.get_sub_categories(None).await?;

    assert_eq!(query.count(), 1);

    let mutation = conn.delete_category(id).await?;

    assert_eq!(mutation, id);

    Ok(())
}
