pub mod errors;
pub(crate) mod mutation;
pub(crate) mod query;

use api_db::DatabaseConnection;
pub use async_graphql::http as async_graphql_extras;
use async_graphql::{extensions::Tracing, EmptySubscription, Schema};

use self::{errors::schema::SchemaError, mutation::Mutation, query::Query};

pub type GraphQLSchema = Schema<Query, Mutation, EmptySubscription>;

pub async fn create_schema() -> Result<GraphQLSchema, SchemaError> {
    let database = DatabaseConnection::from_env().await?;
    let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(database)
        .extension(Tracing);

    #[cfg(not(debug_assertions))]
    return Ok(schema.disable_introspection().finish());

    #[cfg(debug_assertions)]
    return Ok(schema.finish());
}
