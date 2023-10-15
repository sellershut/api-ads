pub(crate) mod cache_keys;
pub mod mutation;
pub mod query;
pub(crate) mod redis;

use api_core::category::Category;
use surrealdb::sql::Thing;

lazy_static::lazy_static! {
    static ref CATEGORY_COLLECTION: &'static str = "category";
}

pub(crate) fn map_err(err: impl ToString) -> String {
    crate::log_error!(err.to_string())
}

#[derive(Debug, serde::Deserialize)]
struct InternalCategory {
    id: Thing, //need to convert thing to string before we send it
    name: String,
    sub_categories: Vec<Thing>, // no heap allocation on empty vecs
    image_url: Option<String>,
    is_root: bool,
}

#[derive(Debug, serde::Serialize)]
struct InsertCategory {
    name: String,
    sub_categories: Vec<String>,
    image_url: Option<String>,
    is_root: bool,
}

impl From<Category> for InsertCategory {
    fn from(mut value: Category) -> Self {
        Self {
            name: std::mem::take(&mut value.name),
            sub_categories: std::mem::take(&mut value.sub_categories),
            image_url: std::mem::take(&mut value.image_url),
            is_root: value.is_root,
        }
    }
}

impl From<InternalCategory> for Category {
    fn from(mut value: InternalCategory) -> Self {
        Self {
            id: value.id.to_string(),
            name: std::mem::take(&mut value.name),
            sub_categories: std::mem::take(&mut value.sub_categories)
                .iter()
                .map(|f| f.id.to_string())
                .collect::<Vec<_>>(),
            image_url: std::mem::take(&mut value.image_url),
            is_root: value.is_root,
            ..Default::default()
        }
    }
}

#[macro_export]
macro_rules! log_error {
    ($x: expr) => {{
        let msg = format!("{}", $x);
        tracing::error!("{msg}");
        msg
    }};
}
