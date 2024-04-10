//! basic User info `Kréta` stores
use std::{collections::HashMap, fmt};

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
            "Intezmeny: {}",
            self.extra
                .get("IntezmenyNev")
                .expect("couldn't get institute name")
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
            "Szuletesi datum: {}",
            self.extra
                .get("SzuletesiDatum")
                .expect("couldn't get birth date")
        )?;

        Ok(())
    }
}
