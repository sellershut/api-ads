use async_trait::async_trait;

pub mod category {
    use crate::category::Category;

    use super::*;

    #[async_trait]
    pub trait Query {
        async fn get_categories(&self) -> Result<Vec<Category>, String>;
        async fn get_sub_categories(
            &self,
            parent_id: Option<&str>,
        ) -> Result<Vec<Category>, String>;
        async fn get_category_by_id(&self, id: &str) -> Result<Option<Category>, String>;
    }
}
