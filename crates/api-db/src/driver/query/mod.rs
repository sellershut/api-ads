use std::str::FromStr;

use api_core::{category::Category, engine::query::category};
use async_trait::async_trait;
use surrealdb::sql::Thing;

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
        let id = Thing::from_str(id).map_err(|_| format!("id {id} could not be parsed"))?;

        self.database.select(id).await.map_err(map_err)
    }

    async fn get_sub_categories(&self, parent_id: Option<&str>) -> Result<Vec<Category>, String> {
        let mut resp = self
            .database
            .query(format!(
                "SELECT * FROM {} WHERE parent_id {}",
                *CATEGORY_COLLECTION,
                if let Some(id) = parent_id {
                    self.database
                        .set(
                            "parent_id",
                            if let Some(last) = id.split(':').last() {
                                last
                            } else {
                                id
                            },
                        )
                        .await
                        .map_err(map_err)?;
                    "= $parent_id"
                } else {
                    "IS null"
                }
            ))
            .await
            .map_err(map_err)?;

        resp.take(0).map_err(map_err)
    }
}
