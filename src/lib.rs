pub mod api;
pub mod api_urls;
pub mod args;
pub mod info;
pub mod messages;
pub mod school_list;
pub mod timetable;
pub mod token;

/// Result from `T` and `Box<dyn Error>`
pub type AnyErr<T> = Result<T, Box<dyn std::error::Error>>;
