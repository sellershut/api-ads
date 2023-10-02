use std::str::FromStr;

use api_core::{category::Category, engine::mutation::category};
use async_trait::async_trait;
use surrealdb::sql::Thing;

use crate::DatabaseConnection;

use super::{map_err, CATEGORY_COLLECTION};

#[async_trait]
impl category::Mutation for DatabaseConnection {
    async fn create_category(&self, content: Category) -> Result<Category, String> {
        let item: Vec<Category> = self
            .surreal
            .create(*CATEGORY_COLLECTION)
            .content(content)
            .await
            .map_err(map_err)?;

        let item = item.into_iter().nth(0);
        item.ok_or("Could not get query result".into())
    }

    async fn update_category(&self, id: &str, content: Category) -> Result<Category, String> {
        let id =
            Thing::from_str(id).map_err(|_| "could not convert id to internal type".to_string())?;

        let item: Option<Category> = self
            .surreal
            .update(id)
            .content(content)
            .await
            .map_err(map_err)?;

        let item = item.into_iter().nth(0);
        item.ok_or("Could not get query result".into())
    }

    async fn delete_category<'a>(&self, id: &'a str) -> Result<&'a str, String> {
        let id_thing =
            Thing::from_str(id).map_err(|_| "could not convert id to internal type".to_string())?;

        let item: Option<Category> = self.surreal.delete(id_thing).await.map_err(map_err)?;
        if let Some(_item) = item {
            Ok(id)
        } else {
            Err("nothing to delete".to_owned())
        }
    }
}
