pub mod mutation;
pub mod query;

lazy_static::lazy_static! {
    static ref CATEGORY_COLLECTION: &'static str = "category";
}

fn map_err(err: impl ToString) -> String {
    err.to_string()
}
