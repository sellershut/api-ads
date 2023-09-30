use async_trait::async_trait;

pub mod category {
    use crate::category::Category;

    use super::*;

    #[async_trait]
    pub trait Mutation {
        async fn create_category(&self, content: Category) -> Result<Category, String>;
        async fn update_category(&self, id: &str, content: Category) -> Result<Category, String>;
        async fn delete_category<'a>(&self, id: &'a str) -> Result<&'a str, String>;
    }
}
