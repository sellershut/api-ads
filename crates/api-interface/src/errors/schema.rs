use thiserror::Error;

#[derive(Error, Debug)]
pub enum SchemaError {
    #[error("could not establish database connection")]
    Database(#[from] api_db::errors::DatabaseError),
    #[error("unknown schema error")]
    Unknown,
}
