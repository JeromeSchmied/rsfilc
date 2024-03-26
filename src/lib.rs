pub mod kreta_api;

/// Result from `T` and `Box<dyn Error>`
pub type AnyErr<T> = Result<T, Box<dyn std::error::Error>>;
