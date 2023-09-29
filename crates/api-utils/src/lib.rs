pub fn unwrap_env_variable(variable: &str) -> Option<String> {
    std::env::var(variable)
        .ok()
        .and_then(|f| if f.trim().is_empty() { None } else { Some(f) })
}
