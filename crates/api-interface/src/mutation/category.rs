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

    async fn update_category(
        &self,
        ctx: &Context<'_>,
        id: String,
        input: Category,
    ) -> async_graphql::Result<Category> {
        let database = ctx.data::<DatabaseConnection>()?;

        match database.update_category(&id, input).await {
            Ok(val) => {
                SimpleBroker::publish(CategoryChanged {
                    mutation_type: MutationType::Updated,
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

    async fn delete_category(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> async_graphql::Result<String> {
        let database = ctx.data::<DatabaseConnection>()?;

        match database.delete_category(&id).await {
            Ok(val) => {
                SimpleBroker::publish(CategoryChanged {
                    mutation_type: MutationType::Deleted,
                    id: val.into(),
                });
                Ok(id)
            }
            Err(e) => {
                error!("{e}");
                Err(e.into())
            }
        }
    }
}
