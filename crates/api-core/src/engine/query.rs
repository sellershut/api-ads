use async_trait::async_trait;

pub mod category {
    use crate::category::Category;

    use super::*;

    #[async_trait]
    pub trait Query {
        type Iter: ExactSizeIterator<Item = Category>;

        async fn get_categories(&self) -> Result<Self::Iter, String>;
        async fn get_sub_categories(&self, parent_id: Option<&str>) -> Result<Self::Iter, String>;
        async fn get_category_by_id(&self, id: &str) -> Result<Option<Category>, String>;
    }
}
