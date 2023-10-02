use std::fmt::Display;

pub enum CacheKey<'a> {
    AllCategories,
    SubCategories { parent: &'a str },
    Category { id: &'a str },
}

impl Display for CacheKey<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CacheKey::AllCategories => write!(f, "categories:all"),
            CacheKey::SubCategories { parent } => write!(f, "categories:parent={parent}"),
            CacheKey::Category { id } => write!(f, "categories:id={id}"),
        }
    }
}
