use api_core::{category::Category, engine::query::category::Query};
use api_db::DatabaseConnection;
use async_graphql::{Context, Object, Result};

use super::{relay::paginate, ConnectionResult, Params};

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
        let database = ctx.data::<DatabaseConnection>()?;

        let categories = database.get_categories().await?;

        let p = Params::new(after, before, first, last);
        paginate(categories.into_iter(), p, 100).await
    }

    async fn category_by_id(&self, ctx: &Context<'_>, id: String) -> Result<Option<Category>> {
        let database = ctx.data::<DatabaseConnection>()?;

        database
            .get_category_by_id(&id)
            .await
            .map_err(async_graphql::Error::from)
    }
}
