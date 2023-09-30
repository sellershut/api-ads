use api_core::{category::Category, engine::query::category};
use async_trait::async_trait;

use crate::DatabaseConnection;

use super::{map_err, CATEGORY_COLLECTION};

#[async_trait]
impl category::Query for DatabaseConnection {
    async fn get_categories(&self) -> Result<Vec<Category>, String> {
        self.database
            .select(*CATEGORY_COLLECTION)
            .await
            .map_err(map_err)
    }
    async fn get_category_by_id(&self, id: &str) -> Result<Option<Category>, String> {
        // check redis and set redis if unavailable
        todo!()
    }

    async fn get_sub_categories(&self, parent_id: Option<&str>) -> Result<Vec<Category>, String> {
        todo!()
    }
}
