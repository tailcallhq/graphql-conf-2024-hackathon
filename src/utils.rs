use std::str::FromStr;

pub fn env_default<T: FromStr>(name: &str, default_value: T) -> T {
    std::env::var(name)
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(default_value)
}
