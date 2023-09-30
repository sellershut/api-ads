use api_core::category::Category;
use api_db::DatabaseConnection;
use async_graphql::{Context, Object};

#[derive(Default)]
pub struct CategoryMutation;

#[Object]
impl CategoryMutation {
    async fn create_category(
        &self,
        ctx: &Context<'_>,
        input: Category,
    ) -> async_graphql::Result<Vec<Category>> {
        let database = ctx.data::<DatabaseConnection>()?;

        todo!()
    }
}
