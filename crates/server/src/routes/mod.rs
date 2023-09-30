use api_interface::{
    async_graphql_extras::{playground_source, GraphQLPlaygroundConfig},
    GraphQLSchema,
};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    response::{Html, IntoResponse},
    Extension,
};

pub(crate) async fn graphql_handler(
    schema: Extension<GraphQLSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

pub(crate) async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(
        GraphQLPlaygroundConfig::new("/").subscription_endpoint("/ws"),
    ))
}
