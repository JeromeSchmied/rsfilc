//! every school that uses the `Kréta` system

use crate::{cache, fill, Res};
use log::info;
use std::fmt::Write;

pub fn handle(search: Option<String>) -> Res<()> {
    let mut schools = fetch()?;
    if let Some(school_name) = search {
        // let found = School::search(&schools, &school_name);
        filter(&mut schools, &school_name);
        for school in schools {
            let as_str = disp(&school);
            println!("\n\n{as_str}");
            fill(&as_str, '-', None);
        }
    } else {
        info!("listing schools");
        for school in schools {
            let as_str = disp(&school);
            println!("\n\n{as_str}");
            fill(&as_str, '-', None);
        }
    }
    Ok(())
}

pub fn fetch() -> Res<Vec<ekreta::School>> {
    let cached = cache::load("schools");
    if let Some((_t, content)) = cached {
        log::info!("loading schools from cache");
        let cached_schools = serde_json::from_str(&content)?;
        return Ok(cached_schools);
    }
    let resp = ekreta::School::fetch_schools_resp()?;

    log::info!("recieved schools from refilc api");
    let json = &resp.text()?;
    let vec = serde_json::from_str(json)?;
    cache::store("schools", json)?;
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
