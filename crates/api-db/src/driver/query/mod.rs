use api_core::{category::Category, engine::query::category::Query, ProtobufMessage};
use async_trait::async_trait;
use redis::AsyncCommands;
use std::str::FromStr;
use surrealdb::sql::Thing;

use crate::{driver::redis::redis_set, DatabaseConnection};

use super::{cache_keys::CacheKey, map_err, InternalCategory, CATEGORY_COLLECTION};

async fn validate_cache(database: &DatabaseConnection, cache_key: &str) -> Result<Vec<u8>, String> {
    let mut redis_conn = database.redis.get().await.map_err(map_err)?;
    redis_conn
        .get::<_, Vec<u8>>(cache_key)
        .await
        .map_err(map_err)
        .and_then(|f| {
            if f.is_empty() {
                Err("empty cache".to_string())
            } else {
                Ok(f)
            }
        })
}

#[async_trait]
impl Query for DatabaseConnection {
    async fn get_categories(
        &self,
    ) -> Result<Box<dyn ExactSizeIterator<Item = Category> + Send + Sync>, String> {
        let cache_key = CacheKey::AllCategories.to_string();

        match validate_cache(self, &cache_key).await {
            Ok(ref val) => {
                let cache_data: Vec<Category> = bincode::deserialize(val).map_err(map_err)?;
                Ok(Box::new(cache_data.into_iter()))
            }

            Err(e) => {
                tracing::warn!(cache_key = %cache_key, "{e}");
                let items: Vec<InternalCategory> = self
                    .surreal
                    .select(*CATEGORY_COLLECTION)
                    .await
                    .map_err(map_err)?;
                println!("Here");

                let mut item = items.into_iter().map(Category::from);

                let data: Vec<Category> = item.by_ref().collect();

                let ex = self.cache_ttl;

                match bincode::serialize(&data) {
                    Ok(data) => {
                        redis_set(data, ex, cache_key, self.redis.clone()).await;
                    }
                    Err(e) => {
                        tracing::error!("{e}");
                    }
                }

                Ok(Box::new(data.into_iter()))
            }
        }
    }

    async fn get_category_by_id(&self, id: &str) -> Result<Option<Category>, String> {
        let cache_key = CacheKey::Category { id }.to_string();

        match validate_cache(self, &cache_key).await {
            Ok(bytes) => {
                let category = Category::parse_from_bytes(&bytes).map_err(map_err)?;
                Ok(Some(category))
            }
            Err(e) => {
                tracing::warn!("{e}");
                let id_internal = Thing::from_str(id)
                    .map_err(|_| map_err(format!("id {id} could not be parsed")))?;

                let category: Option<InternalCategory> =
                    self.surreal.select(id_internal).await.map_err(map_err)?;

                if let Some(category) = category {
                    let category = Category::from(category);
                    let bytes = category.write_to_bytes().map_err(map_err)?;
                    let ex = self.cache_ttl;
                    redis_set(bytes, ex, cache_key, self.redis.clone()).await;
                    Ok(Some(category))
                } else {
                    Ok(None)
                }
            }
        }
    }

    async fn get_sub_categories<'a>(
        &self,
        parent_id: Option<&'a str>,
    ) -> Result<Box<dyn ExactSizeIterator<Item = Category> + Send + Sync>, String> {
        let cache_key = CacheKey::SubCategories {
            parent: parent_id.unwrap_or_default(),
        }
        .to_string();

        match validate_cache(self, &cache_key).await {
            Ok(ref val) => {
                let cache_data: Vec<Category> = bincode::deserialize(val).map_err(map_err)?;
                Ok(Box::new(cache_data.into_iter()))
            }

            Err(e) => {
                tracing::warn!(cache_key = %cache_key, "{e}");
                let mut resp = self
                    .surreal
                    .query(if let Some(parent) = parent_id {
                        format!("SELECT sub_categories.*.* FROM {parent};")
                    } else {
                        format!("SELECT * FROM {} WHERE is_root=true", *CATEGORY_COLLECTION)
                    })
                    .await
                    .map_err(map_err)?;

                let items: Vec<InternalCategory> = resp.take(0).map_err(map_err)?;

                let mut item = items.into_iter().map(Category::from);
                let data: Vec<Category> = item.by_ref().collect();

                let ex = self.cache_ttl;
                match bincode::serialize(&data) {
                    Ok(data) => {
                        redis_set(data, ex, cache_key, self.redis.clone()).await;
                    }
                    Err(e) => {
                        tracing::error!("{e}");
                    }
                }

                Ok(Box::new(data.into_iter()))
            }
        }
    }
}
