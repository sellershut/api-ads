use crate::create_schema;

#[tokio::test]
async fn gql_query() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let schema = create_schema().await?;

    let res = schema
        .execute(
            r#"
           query {
             categories(first: 2) {
               edges{
                 cursor
                 node{
                   id,
                   name
                 }
               },
               pageInfo {
                 hasNextPage,
                 hasPreviousPage
               }
             }
           }
           "#,
        )
        .await;

    assert!(res.errors.is_empty());

    Ok(())
}

#[tokio::test]
async fn gql_query_sub_categories_ok() {
    dotenvy::dotenv().ok();
    let schema = create_schema().await.unwrap();
    let res = schema
        .execute(
            r#"
           query {
             subCategories(first: 2) {
               edges{
                 cursor
                 node{
                   id,
                   name
                 }
               },
               pageInfo {
                 hasNextPage,
                 hasPreviousPage
               }
             }
           }
           "#,
        )
        .await;

    assert!(res.errors.is_empty());
}

#[tokio::test]
async fn gql_mutation_category() {
    dotenvy::dotenv().ok();
    let schema = create_schema().await.unwrap();
    use fake::{faker::lorem::en::Word, Fake};
    let name = format!("\"{}\"", Word().fake::<String>());

    let res = schema
        .execute(format!(
            r"
            mutation {{
              createCategory(input: {{ name: {name}, subCategories: [], isRoot: true}}) {{
                id
              }}
            }}
            ",
        ))
        .await;

    dbg!(&res);

    assert!(res.errors.is_empty());
}
