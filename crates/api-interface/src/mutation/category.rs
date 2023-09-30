use api_core::{category::Category, engine::mutation::category::Mutation};
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
    ) -> async_graphql::Result<Category> {
        let database = ctx.data::<DatabaseConnection>()?;

        database
            .create_category(input)
            .await
            .map_err(async_graphql::Error::from)
    }
}
