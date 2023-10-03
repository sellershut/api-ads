use crate::category::Category;

#[test]
fn serialise_category_proto() {
    let mut category = Category::new();
    category.name = "foo".to_string();
    category.id = "bar".to_string();
    assert!(serde_json::to_string(&category).is_ok());
}
