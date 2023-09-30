use async_graphql::ID;

use crate::mutation::MutationType;

pub mod broker;
pub mod category;

#[derive(async_graphql::MergedSubscription, Default)]
pub struct Subscription(category::CategorySubscription);

#[derive(Clone)]
pub struct CategoryChanged {
    pub mutation_type: MutationType,
    pub id: ID,
}
