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
    /// the token which can be used to refresh stuff
    pub refresh_token: String,

    /// not needed
    #[serde(flatten)]
    extra: HashMap<String, serde_json::Value>,
}
