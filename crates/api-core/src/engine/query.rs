use async_trait::async_trait;

pub mod category {
    use crate::category::Category;

    use super::*;

    #[async_trait]
    pub trait Query {
        async fn get_categories(
            &self,
        ) -> Result<Box<dyn ExactSizeIterator<Item = Category> + Send + Sync>, String>;
        async fn get_sub_categories<'a>(
            &self,
            parent_id: Option<&'a str>,
        ) -> Result<Box<dyn ExactSizeIterator<Item = Category> + Send + Sync>, String>;
        async fn get_category_by_id(&self, id: &str) -> Result<Option<Category>, String>;
    }
}
