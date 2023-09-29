use api_core::category::Category;
use async_graphql::{Context, Object};

use super::{relay::query, ConnectionResult, Params};

#[derive(Default, Debug)]
pub struct CategoryQuery;

#[Object]
impl CategoryQuery {
    async fn categories(
        &self,
        ctx: &Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> ConnectionResult<Category> {
        let items = [1, 2, 3, 4];
        let p = Params::new(after, before, first, last);
        query(items.into_iter(), p, 100).await?
    }
}
