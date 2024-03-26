use crate::AnyErr;
use base64::{engine::general_purpose::STANDARD, Engine};
use hmac::{Hmac, Mac};
use reqwest::header::HeaderMap;
use serde::Deserialize;
use serde_json::Value;
use sha2::Sha512;
use std::{cmp::Ordering, collections::HashMap, fmt};

pub mod admin_endpoints;
pub mod endpoints;

pub fn base(school_id: &str) -> String {
    format!("https://{school_id}.e-kreta.hu")
}

const IDP: &str = "https://idp.e-kreta.hu";
const ADMIN: &str = "https://eugyintezes.e-kreta.hu";
const FILES: &str = "https://files.e-kreta.hu";
const USER_AGENT: &str = "hu.ekreta.student/1.0.4/Android/0/0";
const CLIENT_ID: &str = "kreta-ellenorzo-mobile-android";

#[derive(Deserialize)]
pub struct Token {
    pub access_token: String,
    pub refresh_token: String,

    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

pub struct User {
    user_name: String,
    password: String,
    school_id: String,
}
impl User {
    pub fn new(user_name: &str, password: &str, school_id: &str) -> Self {
        Self {
            user_name: user_name.to_string(),
            password: password.to_string(),
            school_id: school_id.to_string(),
        }
    }

    pub async fn get_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            format!("Bearer {}", self.get_token().await.unwrap().access_token)
                .parse()
                .unwrap(),
        );
        headers.insert("User-Agent", USER_AGENT.parse().unwrap());
        headers
    }

    /// get access token from credentials, school_id
    ///
    /// ```shell
    /// curl "https://idp.e-kreta.hu/connect/token"
    /// -A "hu.ekreta.tanulo/1.0.5/Android/0/0"
    /// -H "X-AuthorizationPolicy-Key: xxx"
    /// -H "X-AuthorizationPolicy-Version: v2"
    /// -H "X-AuthorizationPolicy-Nonce: xxx"
    /// -d "userName=xxxxxxxx&password=xxxxxxxxx&institute_code=xxxxxxxxx&grant_type=password&client_id=kreta-ellenorzo-mobile-android"
    /// ```
    pub async fn get_token(&self) -> AnyErr<Token> {
        // Define the key as bytes
        let key: &[u8] = &[98, 97, 83, 115, 120, 79, 119, 108, 85, 49, 106, 77];
        let nonce = reqwest::get([IDP, endpoints::NONCE].concat())
            .await?
            .text()
            .await?;

        // Define the message as bytes
        let message = format!(
            "{}{}{}",
            self.school_id.to_uppercase(),
            nonce,
            self.user_name.to_uppercase()
        );

        // Create a new HMAC instance
        let mut mac = Hmac::<Sha512>::new_from_slice(key)?;

        // Update the MAC with the message
        mac.update(message.as_bytes());

        // Obtain the result of the MAC computation
        let result = mac.finalize();

        // Encode the result in base64
        let generated = STANDARD.encode(result.into_bytes());

        let mut headers = HeaderMap::new();

        headers.insert(
            "Content-Type",
            "application/x-www-form-urlencoded; charset=utf-8"
                .parse()
                .unwrap(),
        );
        headers.insert("User-Agent", USER_AGENT.parse().unwrap());
        headers.insert("X-AuthorizationPolicy-Key", generated.parse().unwrap());
        headers.insert("X-AuthorizationPolicy-Version", "v2".parse().unwrap());
        headers.insert("X-AuthorizationPolicy-Nonce", nonce.parse().unwrap());

        let mut data = HashMap::new();
        data.insert("userName", self.user_name.as_str());
        data.insert("password", &self.password);
        data.insert("institute_code", &self.school_id);
        data.insert("grant_type", "password");
        data.insert("client_id", CLIENT_ID);

        // eprintln!("sending data: {:?}", data);

        let client = reqwest::Client::new();
        let res = client
            .post([IDP, endpoints::TOKEN].concat())
            .headers(headers)
            .form(&data)
            .send()
            .await?;

        let token = serde_json::from_str(&res.text().await?)?;
        Ok(token)
    }

    /// get user info
    pub async fn get_info(&self) -> AnyErr<Value> {
        let client = reqwest::Client::new();
        let res = client
            .get(base(&self.school_id) + endpoints::STUDENT)
            .headers(self.get_headers().await)
            .send()
            .await?;

        let val = serde_json::from_str(&res.text().await?)?;
        Ok(val)
    }

    /// get messages
    pub async fn get_messages(&self, message_kind: MessageKind) -> AnyErr<String> {
        let client = reqwest::Client::new();
        let res = client
            .get(base(&self.school_id) + &admin_endpoints::get_message(&message_kind.val()))
            .headers(self.get_headers().await)
            .send()
            .await?;

        // let val = serde_json::from_str(&res.text().await?)?;
        // Ok(val)
        Ok(res.text().await?)
    }

    /// get evaluations
    pub async fn get_evals(&self) -> AnyErr<Value> {
        let client = reqwest::Client::new();
        let res = client
            .get(base(&self.school_id) + endpoints::EVALUATIONS)
            .headers(self.get_headers().await)
            .send()
            .await?;

        let val = serde_json::from_str(&res.text().await?)?;
        Ok(val)
    }

    /// get timetable
    pub async fn get_timetable(&self, from: Time, to: Time) -> AnyErr<Value> {
        eprintln!("from: {}\nto: {}", from, to);

        let client = reqwest::Client::new();
        let res = client
            .get(base(&self.school_id) + endpoints::TIMETABLE)
            // .query(&[("datumTol", from.to_string()), ("datumIg", to.to_string())])
            .query(&[
                ("datumTol", "2024-03-18T00:00:00".to_owned()),
                ("datumIg", "2024-03-24T00:00:00".to_owned()),
            ])
            .headers(self.get_headers().await)
            .send()
            .await?;

        let val = serde_json::from_str(&res.text().await?)?;
        Ok(val)
    }
}

/// kinds of message
pub enum MessageKind {
    Beerkezett,
    Elkuldott,
    Torolt,
}
impl MessageKind {
    pub fn val(&self) -> String {
        match self {
            MessageKind::Beerkezett => "beerkezett".to_owned(),
            MessageKind::Elkuldott => "elkuldott".to_owned(),
            MessageKind::Torolt => "torolt".to_owned(),
        }
    }
}

/// 2020-09-08T00-00-00
#[derive(Debug)]
pub struct Time {
    year: u16,
    month: u8,
    day: u8,

    hour: u8,
    min: u8,
    sec: u8,
}
impl Time {
    pub fn new_all(year: u16, month: u8, day: u8, hour: u8, min: u8, sec: u8) -> Self {
        assert!(year > 1800);
        assert!((1..=12).contains(&month));
        assert!((1..=31).contains(&day));
        assert!(hour <= 24);
        assert!(min <= 60);
        assert!(sec <= 60);
        Self {
            year,
            month,
            day,
            hour,
            min,
            sec,
        }
    }
    pub fn new(year: u16, month: u8, day: u8) -> Self {
        assert!(year > 1800);
        assert!((1..=12).contains(&month));
        assert!((1..=31).contains(&day));
        Self {
            year,
            month,
            day,
            hour: 0,
            min: 0,
            sec: 0,
        }
    }
    /// fill with 0 if needed
    fn fill(t: u16) -> String {
        match t.cmp(&10) {
            Ordering::Greater | Ordering::Equal => t.to_string(),
            Ordering::Less => format!("0{}", t),
        }
    }
}
impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}-{}-{}T{}:{}:{}",
            Self::fill(self.year),
            Self::fill(self.month.into()),
            Self::fill(self.day.into()),
            Self::fill(self.hour.into()),
            Self::fill(self.min.into()),
            Self::fill(self.sec.into())
        )?;
        Ok(())
    }
}

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

/// ```json
/// {
///   "GlobalMobileApiUrlDEV": "https://kretaglobalmobileapidev.ekreta.hu/",
///   "GlobalMobileApiUrlTEST": "https://kretaglobalmobileapitest.ekreta.hu",
///   "GlobalMobileApiUrlUAT": "https://kretaglobalmobileapiuat.ekreta.hu",
///   "GlobalMobileApiUrlPROD": "https://kretaglobalmobileapi2.ekreta.hu"
/// }    
/// ```
#[derive(Deserialize, Debug)]
pub struct ApiUrls {
    #[serde(rename = "GlobalMobileApiUrlDEV")]
    global_mobile_api_url_dev: String,
    #[serde(rename = "GlobalMobileApiUrlTEST")]
    global_mobile_api_url_test: String,
    #[serde(rename = "GlobalMobileApiUrlUAT")]
    global_mobile_api_url_uat: String,
    #[serde(rename = "GlobalMobileApiUrlPROD")]
    global_mobile_api_url_prod: String,
}
impl ApiUrls {
    pub fn get_api_url() -> String {
        "https://kretamobile.blob.core.windows.net/configuration/ConfigurationDescriptor.json"
            .to_string()
    }
    pub async fn get() -> AnyErr<ApiUrls> {
        let res = reqwest::get(ApiUrls::get_api_url()).await?;

        Ok(serde_json::from_str(&res.text().await?)?)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn api_links_parser() {
        let correct = r#"
    {
       "GlobalMobileApiUrlDEV": "https://kretaglobalmobileapidev.ekreta.hu/",
       "GlobalMobileApiUrlTEST": "https://kretaglobalmobileapitest.ekreta.hu",
       "GlobalMobileApiUrlUAT": "https://kretaglobalmobileapiuat.ekreta.hu",
       "GlobalMobileApiUrlPROD": "https://kretaglobalmobileapi2.ekreta.hu"
    }  "#;
        let apiurls: ApiUrls = serde_json::from_str(correct).unwrap();

        assert_eq!(
            apiurls.global_mobile_api_url_dev,
            String::from("https://kretaglobalmobileapidev.ekreta.hu/")
        );
        assert_eq!(
            apiurls.global_mobile_api_url_test,
            String::from("https://kretaglobalmobileapitest.ekreta.hu")
        );
        assert_eq!(
            apiurls.global_mobile_api_url_uat,
            String::from("https://kretaglobalmobileapiuat.ekreta.hu")
        );
        assert_eq!(
            apiurls.global_mobile_api_url_prod,
            String::from("https://kretaglobalmobileapi2.ekreta.hu")
        );
    }
    #[tokio::test]
    async fn api_url_get() {
        let _apiurls = ApiUrls::get().await.unwrap();
    }
}
