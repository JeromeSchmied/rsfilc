//! every school that uses the `Kréta` system

use crate::{cache, utils, Res};
use log::info;
use std::fmt::Write;

pub fn handle(search: Option<String>) -> Res<()> {
    let mut schools = fetch()?;
    if let Some(school_name) = search {
        filter(&mut schools, &school_name);
    }
    info!("listing schools");
    utils::print_them_basic(schools.iter(), disp);
    Ok(())
}

pub fn fetch() -> Res<Vec<ekreta::School>> {
    let cached = cache::load("", "schools");
    if let Some((_t, content)) = cached {
        log::info!("loading schools from cache");
        let cached_schools = serde_json::from_str(&content)?;
        return Ok(cached_schools);
    }
    let resp = ekreta::School::fetch_schools_resp()?;

    log::info!("received schools from refilc api");
    let json = &resp.into_body().read_to_string()?;
    let vec = serde_json::from_str(json)?;
    cache::store("", "schools", json)?;
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
