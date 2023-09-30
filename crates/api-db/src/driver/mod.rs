use api_core::category::Category;
use surrealdb::sql::Thing;

pub mod mutation;
pub mod query;

lazy_static::lazy_static! {
    static ref CATEGORY_COLLECTION: &'static str = "category";
}

fn map_err(err: impl ToString) -> String {
    err.to_string()
}

#[derive(Debug, serde::Deserialize)]
struct InternalCategory {
    id: Thing, //need to convert thing to string before we send it
    name: String,
    parent_id: Option<String>,
    image_url: Option<String>,
}

impl From<InternalCategory> for Category {
    fn from(mut value: InternalCategory) -> Self {
        Self {
            id: value.id.to_string(),
            name: std::mem::take(&mut value.name),
            parent_id: std::mem::take(&mut value.parent_id),
            image_url: std::mem::take(&mut value.image_url),
            ..Default::default()
        }
    }
}
