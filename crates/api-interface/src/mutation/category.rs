use api_core::{category::Category, engine::mutation::category::Mutation};
use api_db::DatabaseConnection;
use async_graphql::{Context, Object};
use tracing::error;

use crate::subscription::{broker::SimpleBroker, CategoryChanged};

use super::MutationType;

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

        match database.create_category(input).await {
            Ok(val) => {
                SimpleBroker::publish(CategoryChanged {
                    mutation_type: MutationType::Created,
                    id: val.id.clone().into(),
                });
                Ok(val)
            }
            Err(e) => {
                error!("{e}");
                Err(e.into())
            }
        }
    }
}
