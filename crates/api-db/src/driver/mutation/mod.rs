use std::str::FromStr;

use api_core::{category::Category, engine::mutation::category};
use async_trait::async_trait;
use surrealdb::sql::Thing;

use crate::{driver::InsertCategory, DatabaseConnection};

use super::{
    cache_keys::CacheKey, map_err, redis::redis_del, InternalCategory, CATEGORY_COLLECTION,
};

#[async_trait]
impl category::Mutation for DatabaseConnection {
    async fn create_category(&self, content: Category) -> Result<Category, String> {
        let insert: InsertCategory = content.into();
        let item: Vec<InternalCategory> = self
            .surreal
            .create(*CATEGORY_COLLECTION)
            .content(insert)
            .await
            .map_err(map_err)?;

        let item = item.into_iter().nth(0).map(Category::from);
        item.ok_or("Could not get query result".into())
    }

    async fn update_category(&self, id: &str, content: Category) -> Result<Category, String> {
        let id_internal =
            Thing::from_str(id).map_err(|_| "could not convert id to internal type".to_string())?;

        let insert: InsertCategory = content.into();

        let item: Option<InternalCategory> = self
            .surreal
            .update(id_internal)
            .content(insert)
            .await
            .map_err(map_err)?;

        let item = item.into_iter().nth(0).map(Category::from);
        if let Some(ref _item) = item {
            redis_del(CacheKey::Category { id }.to_string(), self.redis.clone()).await;
        }

        item.ok_or("Could not get query result".into())
    }

    async fn delete_category<'a>(&self, id: &'a str) -> Result<&'a str, String> {
        let id_thing =
            Thing::from_str(id).map_err(|_| "could not convert id to internal type".to_string())?;

        let item: Option<InternalCategory> =
            self.surreal.delete(id_thing).await.map_err(map_err)?;

        if let Some(_item) = item {
            redis_del(CacheKey::Category { id }.to_string(), self.redis.clone()).await;
            redis_del(CacheKey::AllCategories.to_string(), self.redis.clone()).await;
            Ok(id)
        } else {
            Err("nothing to delete".to_owned())
        }
    }
}
