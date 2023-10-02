use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("data store disconnected")]
    Connection(#[from] surrealdb::Error),
    #[error("data store disconnected")]
    Redis(#[from] bb8_redis::redis::RedisError),
    #[error("unknown error")]
    Unknown,
}
