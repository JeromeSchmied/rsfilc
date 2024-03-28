use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Info {
    // ora neve
    pub nev: String,

    /// not needed
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}
