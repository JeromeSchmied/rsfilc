//! every school that uses the `Kréta` system

use crate::Res;
use log::info;
use serde::Deserialize;
use std::fmt;

/// a school
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct School {
    city: String,
    institute_code: String,
    name: String,
}
impl School {
    /// get [`School`] list from refilc api
    pub fn get_from_refilc() -> Res<Vec<School>> {
        let res = reqwest::blocking::get("https://api.refilc.hu/v1/public/school-list")?;

        info!("recieved schools from refilc api");
        Ok(serde_json::from_str(&res.text()?)?)
    }
    /// search for school
    pub fn search(schools: &[School], find_school: &str) -> Vec<School> {
        info!("searching for {find_school} in schools");
        let mut matching_schools = Vec::new();
        for school in schools {
            if [
                school.name.to_owned(),
                school.city.to_owned(),
                school.institute_code.to_owned(),
            ]
            .iter()
            .any(|j| j.to_lowercase().contains(&find_school.to_lowercase()))
            {
                matching_schools.push(school.clone());
            }
        }
        matching_schools
    }

    // hey! won't work as it requires header value apiKey to include uppercase K, which is invalid HTTP
    // pub fn get_kreta() -> AnyErr<String> {
    //     // curl "https://kretaglobalmobileapi2.ekreta.hu/api/v3/Institute" -H "apiKey: 7856d350-1fda-45f5-822d-e1a2f3f1acf0"
    //     let client = reqwest::blocking::Client::new();
    //     let res = client
    //         .get("https://kretaglobalmobileapi2.ekreta.hu/api/v3/Institute")
    //         .header("apiKey", "7856d350-1fda-45f5-822d-e1a2f3f1acf0")
    //         .send()?;
    //     warn!("headers: {:?}; status: {}", res.headers(), res.status());
    //     Ok(res.text()?)
    // }
}

impl fmt::Display for School {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "| {}", self.name.replace('"', ""))?;
        writeln!(f, "| id: {}", self.institute_code)?;
        write!(f, "| helye: {}", self.city)?;

        Ok(())
    }
}
