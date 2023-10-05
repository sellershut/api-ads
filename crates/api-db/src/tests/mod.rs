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
