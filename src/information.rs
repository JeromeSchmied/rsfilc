//! basic User info, `Kréta` stores

use chrono::{DateTime, Local};
use serde::Deserialize;
use std::{collections::HashMap, fmt};

#[derive(Debug, Deserialize)]
/// basic user info
pub struct Info {
    // name of the student
    #[serde(rename(deserialize = "Nev"))]
    pub name: String,

    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}
impl fmt::Display for Info {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "| {}", self.name)?;
        writeln!(
            f,
            "| Intézmény: {}",
            self.extra
                .get("IntezmenyNev")
                .unwrap()
                .to_string()
                .trim_matches('"')
        )?;
        writeln!(
            f,
            "|   id: {}",
            self.extra.get("IntezmenyAzonosito").unwrap()
        )?;
        write!(
            f,
            "| Születési dátum: {}",
            DateTime::parse_from_rfc3339(
                self.extra
                    .get("SzuletesiDatum")
                    .unwrap()
                    .to_string()
                    .trim_matches('"')
            )
            .unwrap()
            .with_timezone(&Local)
            .format("%Y.%m.%d")
        )?;

        Ok(())
    }
}
