//! Bearer token, that's used to access any user-related data

use serde::Deserialize;
use std::collections::HashMap;

/// Token
///
/// consists of
/// - `access_token`
/// - `refresh_token`
/// - extra stuff, not needed
#[derive(Deserialize, Debug)]
pub struct Token {
    /// the bearer token
    pub access_token: String,
    /// the token which can be used to refresh the bearer token
    pub refresh_token: String,

    /// not needed
    #[serde(flatten)]
    _extra: HashMap<String, serde_json::Value>,
}
impl Token {
    /// access [`Token`] endpoint
    pub const fn ep() -> &'static str {
        "/connect/token"
    }
}
