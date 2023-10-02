use api_core::category::Category;
use itertools::Either;
use redis::AsyncCommands;
use std::str::FromStr;
use surrealdb::sql::Thing;
use tracing::Instrument;

use crate::DatabaseConnection;

use super::{cache_keys::CacheKey, map_err, InternalCategory, CATEGORY_COLLECTION};

impl DatabaseConnection {
    pub async fn get_categories(
        &self,
    ) -> Result<
        Either<impl ExactSizeIterator<Item = Category>, impl ExactSizeIterator<Item = Category>>,
        String,
    > {
        let cache_key = CacheKey::AllCategories.to_string();

        let mut redis_conn = self.redis.get().await.map_err(map_err)?;

        match redis_conn
            .get::<_, Vec<u8>>(&cache_key)
            .await
            .map_err(map_err)
            .and_then(|f| {
                if f.is_empty() {
                    Err("empty cache".to_string())
                } else {
                    Ok(f)
                }
            }) {
            Ok(ref val) => {
                let cache_data: Vec<Category> = bincode::deserialize(val).map_err(map_err)?;
                Ok(Either::Left(cache_data.into_iter()))
            }

            Err(e) => {
                tracing::warn!(cache_key = %cache_key, "{e}");
                let items: Vec<InternalCategory> = self
                    .surreal
                    .select(*CATEGORY_COLLECTION)
                    .await
                    .map_err(map_err)?;

                let mut item = items.into_iter().map(Category::from);

                let data: Vec<Category> = item.by_ref().collect();

                let redis_conn = self.redis.clone();
                let fut = async move {
                    let mut conn = redis_conn.get().await.map_err(map_err).unwrap();
                    if let Ok(data) = bincode::serialize(&data) {
                        let redis: Result<(), redis::RedisError> =
                            conn.set_ex(cache_key, data, 300).await;
                        if let Err(e) = redis {
                            tracing::error!("{e}");
                        }
                    }
                }
                .instrument(tracing::debug_span!("redis.set"));

                #[cfg(feature = "tokio")]
                {
                    tokio::spawn(fut);
                }

                #[cfg(not(feature = "tokio"))]
                fut.await;

                Ok(Either::Right(item))
            }
        }
    }

    pub async fn get_category_by_id(&self, id: &str) -> Result<Option<Category>, String> {
        let id = Thing::from_str(id).map_err(|_| format!("id {id} could not be parsed"))?;

        self.surreal.select(id).await.map_err(map_err)
    }

    pub async fn get_sub_categories(
        &self,
        parent_id: Option<&str>,
    ) -> Result<impl ExactSizeIterator<Item = Category>, String> {
        let mut resp = self
            .surreal
            .query(format!(
                "SELECT * FROM {} WHERE parent_id {}",
                *CATEGORY_COLLECTION,
                if let Some(id) = parent_id {
                    self.surreal
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
