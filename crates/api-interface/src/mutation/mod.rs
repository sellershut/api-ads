use async_graphql::Enum;

pub mod category;

#[derive(async_graphql::MergedObject, Default)]
pub struct Mutation(category::CategoryMutation);

#[derive(Enum, Eq, PartialEq, Copy, Clone)]
pub enum MutationType {
    Created,
    Deleted,
    Updated,
}
