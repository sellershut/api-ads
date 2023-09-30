use api_core::{category::Category, engine::query::category};
use async_trait::async_trait;
use lazy_static::lazy_static;

use crate::DatabaseConnection;

fn map_err(err: impl ToString) -> String {
    err.to_string()
}

lazy_static! {
    static ref CATEGORY_COLLECTION: &'static str = "category";
}

#[async_trait]
impl category::Query for DatabaseConnection {
    async fn get_categories(&self) -> Result<Vec<Category>, String> {
        self.database
            .select(*CATEGORY_COLLECTION)
            .await
            .map_err(map_err)
    }
    async fn get_category_by_id(&self, id: &str) -> Result<Category, String> {
        // check redis and set redis if unavailable
        todo!()
    }

    async fn get_sub_categories(&self, parent_id: Option<&str>) -> Result<Vec<Category>, String> {
        todo!()
    }
}
