use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("data store disconnected")]
    Connection(#[from] surrealdb::Error),
    #[error("unknown error")]
    Unknown,
}
