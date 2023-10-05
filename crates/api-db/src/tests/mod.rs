use api_core::{category::Category, engine::query::category::Query};
use async_trait::async_trait;
use mockall::mock;

mock! {
    DatabaseConnection {}     // Name of the mock struct, less the "Mock" prefix

    #[async_trait]
    impl Query for DatabaseConnection {   // specification of the trait to mock
        async fn get_categories(&self) -> Result<Box<dyn ExactSizeIterator<Item=Category> + Send+Sync>, String>;
        async fn get_sub_categories<'a>(&self, parent_id: Option<&'a str>) -> Result<Box<dyn ExactSizeIterator<Item=Category> + Send+Sync>, String>;
        async fn get_category_by_id(&self, id: &str) -> Result<Option<Category>, String>;
    }
}

async fn check(input: &[Category]) {
    let mut mock = MockDatabaseConnection::new();
    mock.expect_get_categories()
        .returning(|| Ok(Box::new(vec![].into_iter())));

    let result = mock.get_categories().await;

    assert!(result.is_ok());

    assert_eq!(result.unwrap().count(), input.len());
}

#[tokio::test]
async fn get_categories() {
    let items: Vec<Category> = vec![];
    check(&items).await;
}
