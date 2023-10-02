use bb8::Pool;
use bb8_redis::RedisMultiplexedConnectionManager;
use redis::AsyncCommands;

use super::map_err;

#[tracing::instrument(skip(value))]
pub async fn redis_set(
    value: impl serde::Serialize + Send + Sync + 'static,
    ex: usize,
    cache_key: String,
    conn: Pool<RedisMultiplexedConnectionManager>,
) {
    let fut = async move {
        let mut conn = conn.get().await.map_err(map_err).unwrap();
        if let Ok(data) = bincode::serialize(&value) {
            let redis: Result<(), redis::RedisError> = conn.set_ex(cache_key, data, ex).await;
            if let Err(e) = redis {
                tracing::error!("{e}");
            }
        }
    };

    #[cfg(feature = "tokio")]
    {
        tokio::spawn(fut);
    }

    #[cfg(not(feature = "tokio"))]
    fut.await;
}
