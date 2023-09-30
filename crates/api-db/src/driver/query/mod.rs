use api_core::category::Category;
use std::str::FromStr;
use surrealdb::sql::Thing;

use crate::DatabaseConnection;

use super::{map_err, InternalCategory, CATEGORY_COLLECTION};

impl DatabaseConnection {
    pub async fn get_categories(&self) -> Result<impl ExactSizeIterator<Item = Category>, String> {
        let items: Vec<InternalCategory> = self
            .database
            .select(*CATEGORY_COLLECTION)
            .await
            .map_err(map_err)?;

        let item = items.into_iter().map(Category::from);
        Ok(item)
    }

    pub async fn get_category_by_id(&self, id: &str) -> Result<Option<Category>, String> {
        let id = Thing::from_str(id).map_err(|_| format!("id {id} could not be parsed"))?;

        self.database.select(id).await.map_err(map_err)
    }

    pub async fn get_sub_categories(
        &self,
        parent_id: Option<&str>,
    ) -> Result<impl ExactSizeIterator<Item = Category>, String> {
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

        let vecs: Vec<InternalCategory> = resp.take(0).map_err(map_err)?;
        Ok(vecs.into_iter().map(Category::from))
    }
}
