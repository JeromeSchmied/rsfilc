use crate::AnyErr;
use serde::Deserialize;
use std::fmt;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct School {
    city: String,
    institute_code: String,
    name: String,
}
impl School {
    /// get school list from refilc api
    pub async fn get_from_refilc() -> AnyErr<Vec<School>> {
        let res = reqwest::get("https://api.refilc.hu/v1/public/school-list").await?;

        Ok(serde_json::from_str(&res.text().await?)?)
    }
    /// search for school
    pub fn search(find_school: &str, schools: &[School]) -> Vec<School> {
        let mut matching_schools = Vec::new();
        for school in schools {
            if school
                .name
                .to_lowercase()
                .contains(&find_school.to_lowercase())
            {
                matching_schools.push(school.clone());
            }
        }
        matching_schools
    }

    // pub async fn get_kreta() -> Result<String, AnyErr> {
    //     // curl "https://kretaglobalmobileapi2.ekreta.hu/api/v3/Institute" -H "apiKey: 7856d350-1fda-45f5-822d-e1a2f3f1acf0"

    //     let client = reqwest::Client::new();
    //     let res = client
    //         .get("https://kretaglobalmobileapi2.ekreta.hu/api/v3/Institute")
    //         .header("apiKey", "7856d350-1fda-45f5-822d-e1a2f3f1acf0")
    //         .send()
    //         .await?;

    //     // Ok(serde_json::from_str(&res.text().await?)?)
    //     Ok(res.text().await?)
    // }
}
impl fmt::Display for School {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.name.replace('"', ""))?;
        writeln!(f, "id: {}", self.institute_code)?;
        writeln!(f, "location: {}", self.city)?;

        Ok(())
    }
}
