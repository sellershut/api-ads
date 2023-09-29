pub use async_graphql::http as async_graphql_extras;
use async_graphql::{extensions::Tracing, EmptyMutation, EmptySubscription, Schema};

use self::query::Query;

pub(crate) mod query;

pub type GraphQLSchema = Schema<Query, EmptyMutation, EmptySubscription>;

pub fn create_schema() -> GraphQLSchema {
    Schema::build(Query::default(), EmptyMutation, EmptySubscription)
        .extension(Tracing)
        .finish()
}
