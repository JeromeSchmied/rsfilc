use crate::AnyErr;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct School {
    city: String,
    institute_code: String,
    name: String,
}
impl School {
    pub async fn get_from_refilc() -> AnyErr<Vec<School>> {
        let res = reqwest::get("https://api.refilc.hu/v1/public/school-list").await?;

        Ok(serde_json::from_str(&res.text().await?)?)
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
