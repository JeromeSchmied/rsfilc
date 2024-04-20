//! basic User info `Kréta` stores
use std::{collections::HashMap, fmt};

use chrono::{DateTime, Local};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Info {
    // name of the student
    pub nev: String,

    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}
impl fmt::Display for Info {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.nev)?;
        writeln!(
            f,
            "Intézmény: {}",
            self.extra
                .get("IntezmenyNev")
                .expect("couldn't get institute name")
                .to_string()
                .trim_matches('"')
        )?;
        writeln!(
            f,
            "    Id: {}",
            self.extra
                .get("IntezmenyAzonosito")
                .expect("couldn't get institute id")
        )?;
        writeln!(
            f,
            "Születési dátum: {}",
            DateTime::parse_from_rfc3339(
                self.extra
                    .get("SzuletesiDatum")
                    .expect("couldn't get birth date")
                    .to_string()
                    .trim_matches('"')
            )
            .expect("invalid datetime")
            .with_timezone(&Local)
            .format("%Y.%m.%d")
        )?;

        Ok(())
    }
}
