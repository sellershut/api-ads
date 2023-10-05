use api_core::{category::Category, engine::query::category::Query};
use std::time::Duration;

use api_db::DatabaseConnection;
use async_graphql::{Context, Object, Subscription};
use futures_util::{Stream, StreamExt};

use crate::{mutation::MutationType, subscription::CategoryChanged};

use super::broker::SimpleBroker;

#[derive(Default)]
pub struct CategorySubscription;

#[Subscription]
impl CategorySubscription {
    async fn interval(&self, #[graphql(default = 1)] n: i32) -> impl Stream<Item = i32> {
        let mut value = 0;
        async_stream::stream! {
            loop {
                futures_timer::Delay::new(Duration::from_secs(1)).await;
                value += n;
                yield value;
            }
        }
    }

    async fn categories(
        &self,
        mutation_type: Option<MutationType>,
    ) -> impl Stream<Item = CategoryChanged> {
        SimpleBroker::<CategoryChanged>::subscribe().filter(move |event| {
            let res = if let Some(mutation_type) = mutation_type {
                event.mutation_type == mutation_type
            } else {
                true
            };
            async move { res }
        })
    }
}

#[Object]
impl CategoryChanged {
    async fn mutation_type(&self) -> MutationType {
        self.mutation_type
    }

    async fn id(&self) -> String {
        self.id.to_string()
    }

    async fn category(&self, ctx: &Context<'_>) -> async_graphql::Result<Option<Category>> {
        let database = ctx.data::<DatabaseConnection>()?;
        let category = database.get_category_by_id(&self.id).await?;

        Ok(category)
    }
}
