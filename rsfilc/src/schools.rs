//! every school that uses the `Kréta` system

use crate::{cache, uncache, Res};
use ekreta::Endpoint;
use std::fmt::Write;

pub fn fetch() -> Res<Vec<ekreta::School>> {
    let cached = uncache("schools");
    if let Some((_t, content)) = cached {
        log::info!("loading schools from cache");
        let cached_schools = serde_json::from_str(&content)?;
        return Ok(cached_schools);
    }
    let uri = [
        ekreta::School::base_url("").as_ref(),
        ekreta::School::path("").as_str(),
    ]
    .concat();
    let resp = reqwest::blocking::get(uri)?;

    log::info!("recieved schools from refilc api");
    let json = &resp.text()?;
    let vec = serde_json::from_str(json)?;
    cache("schools", json)?;
    Ok(vec)
}

pub fn filter(schools: &mut Vec<ekreta::School>, search_for: &str) {
    log::info!("searching for {search_for} in schools");
    schools.retain(|school| {
        [
            school.nev.clone(),
            school.telepules.clone(),
            school.azonosito.clone(),
        ]
        .iter()
        .any(|j| j.to_lowercase().contains(&search_for.to_lowercase()))
    });
}

pub fn disp(school: &ekreta::School) -> String {
    let mut f = String::new();
    _ = writeln!(&mut f, "| {}", school.nev.replace('"', ""));
    _ = writeln!(&mut f, "| id: {}", school.azonosito);
    _ = write!(&mut f, "| helye: {}", school.telepules);

    f
}
