use bb8::Pool;
use bb8_redis::RedisMultiplexedConnectionManager;
use redis::{AsyncCommands, ToRedisArgs};
use tracing::trace;

use super::map_err;

#[tracing::instrument(skip(value, conn))]
pub async fn redis_set(
    value: impl Send + Sync + 'static + ToRedisArgs,
    ex: usize,
    cache_key: String,
    conn: Pool<RedisMultiplexedConnectionManager>,
) {
    let fut = async move {
        let mut conn = conn.get().await.map_err(map_err).unwrap();
        let redis: Result<(), redis::RedisError> = conn.set_ex(cache_key, value, ex).await;
        if let Err(e) = redis {
            tracing::error!("{e}");
        } else {
            trace!("redis cache updated");
        }
    };

    #[cfg(feature = "tokio")]
    {
        tokio::spawn(fut);
    }

    #[cfg(not(feature = "tokio"))]
    fut.await;
}

#[tracing::instrument(skip(conn))]
pub async fn redis_del(cache_key: String, conn: Pool<RedisMultiplexedConnectionManager>) {
    let fut = async move {
        let mut conn = conn.get().await.map_err(map_err).unwrap();
        let redis: Result<(), redis::RedisError> = conn.del(cache_key).await;
        if let Err(e) = redis {
            tracing::error!("{e}");
        } else {
            trace!("redis cache updated");
        }
    };

    #[cfg(feature = "tokio")]
    {
        tokio::spawn(fut);
    }

    #[cfg(not(feature = "tokio"))]
    fut.await;
}
