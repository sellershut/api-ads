use async_graphql::connection::{Connection, EmptyFields};

use self::relay::{Base64Cursor, ConnectionFields};

pub(crate) mod category;
pub(crate) mod relay;

pub(crate) type ConnectionResult<T> =
    async_graphql::Result<Connection<Base64Cursor, T, ConnectionFields, EmptyFields>>;

#[derive(async_graphql::MergedObject, Default)]
pub struct Query(category::CategoryQuery);

/// Relay-compliant connection parameters to page results by cursor/page size
pub struct Params {
    after: Option<String>,
    before: Option<String>,
    first: Option<i32>,
    last: Option<i32>,
}

impl Params {
    pub const fn new(
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Self {
        Self {
            after,
            before,
            first,
            last,
        }
    }
}
