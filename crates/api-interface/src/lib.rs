pub(crate) mod mutation;
pub(crate) mod query;

pub use async_graphql::http as async_graphql_extras;
use async_graphql::{extensions::Tracing, EmptySubscription, Schema};

use self::{mutation::Mutation, query::Query};

pub type GraphQLSchema = Schema<Query, Mutation, EmptySubscription>;

pub fn create_schema() -> GraphQLSchema {
    Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .extension(Tracing)
        .finish()
}
