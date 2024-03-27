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

    #[serde(flatten)]
    /// not needed
    extra: HashMap<String, serde_json::Value>,
}
