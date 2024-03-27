use crate::{messages::MessageKind, timetable::Lesson, token::Token, AnyErr};
use base64::{engine::general_purpose::STANDARD, Engine};
use hmac::{Hmac, Mac};
use reqwest::header::HeaderMap;
use sha2::Sha512;
use speedate::DateTime;
use std::collections::HashMap;

use crate::api;

mod admin_endpoints;
mod endpoints;

pub fn base(school_id: &str) -> String {
    format!("https://{school_id}.e-kreta.hu")
}

/// kreta idp base Url
const IDP: &str = "https://idp.e-kreta.hu";
/// kreta admin base Url
const ADMIN: &str = "https://eugyintezes.e-kreta.hu";
/// kreta files base Url
const FILES: &str = "https://files.e-kreta.hu";
/// just a random `USER_AGENT`
const USER_AGENT: &str = "hu.ekreta.student/1.0.4/Android/0/0";
/// client id, just like as if it was official
const CLIENT_ID: &str = "kreta-ellenorzo-mobile-android";

/// Kréta user
pub struct User {
    /// the username, usually the `oktatási azonosító szám`: "7" + 10 numbers `7XXXXXXXXXX`
    user_name: String,
    /// the password, usually it defaults to the date of birth of the user: `YYYY-MM-DD`
    password: String,
    /// the id of the school the user goes to, usually looks like:  "klik" + 9 numbers: `klikXXXXXXXXX`
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

    /// get headers which are necessary for making certain requests
    pub async fn headers(&self) -> AnyErr<HeaderMap> {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            format!("Bearer {}", self.token().await?.access_token).parse()?,
        );
        headers.insert("User-Agent", api::USER_AGENT.parse().unwrap());
        Ok(headers)
    }

    /// get `Token` from credentials, school_id
    ///
    /// ```shell
    /// curl "https://idp.e-kreta.hu/connect/token"
    ///     -A "hu.ekreta.tanulo/1.0.5/Android/0/0"
    ///     -H "X-AuthorizationPolicy-Key: xxx"
    ///     -H "X-AuthorizationPolicy-Version: v2"
    ///     -H "X-AuthorizationPolicy-Nonce: xxx"
    ///     -d "userName=xxxxxxxx \
    ///         &password=xxxxxxxxx \
    ///         &institute_code=xxxxxxxxx \
    ///         &grant_type=password \
    ///         &client_id=kreta-ellenorzo-mobile-android"
    /// ```
    pub async fn token(&self) -> AnyErr<Token> {
        // Define the key as bytes
        let key: &[u8] = &[98, 97, 83, 115, 120, 79, 119, 108, 85, 49, 106, 77];

        // Get nonce
        let nonce = reqwest::get([api::IDP, endpoints::NONCE].concat())
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
        headers.insert("User-Agent", api::USER_AGENT.parse().unwrap());
        headers.insert("X-AuthorizationPolicy-Key", generated.parse().unwrap());
        headers.insert("X-AuthorizationPolicy-Version", "v2".parse().unwrap());
        headers.insert("X-AuthorizationPolicy-Nonce", nonce.parse().unwrap());

        let mut data = HashMap::new();
        data.insert("userName", self.user_name.as_str());
        data.insert("password", &self.password);
        data.insert("institute_code", &self.school_id);
        data.insert("grant_type", "password");
        data.insert("client_id", api::CLIENT_ID);

        let client = reqwest::Client::new();
        let res = client
            .post([api::IDP, endpoints::TOKEN].concat())
            .headers(headers)
            .form(&data)
            .send()
            .await?;

        let token = serde_json::from_str(&res.text().await?)?;
        Ok(token)
    }

    /// get user info
    pub async fn info(&self) -> AnyErr<String> {
        let client = reqwest::Client::new();
        let res = client
            .get(base(&self.school_id) + endpoints::STUDENT)
            .headers(self.headers().await?)
            .send()
            .await?;

        // let val = serde_json::from_str(&res.text().await?)?;
        // Ok(val)
        Ok(res.text().await?)
    }

    /// get messages
    pub async fn messages(&self, message_kind: MessageKind) -> AnyErr<String> {
        let client = reqwest::Client::new();
        let res = client
            .get(api::ADMIN.to_owned() + &admin_endpoints::get_message(&message_kind.val()))
            .headers(self.headers().await?)
            .send()
            .await?;

        // let val = serde_json::from_str(&res.text().await?)?;
        // Ok(val)
        Ok(res.text().await?)
    }

    /// get evaluations
    pub async fn evals(&self) -> AnyErr<String> {
        let client = reqwest::Client::new();
        let res = client
            .get(base(&self.school_id) + endpoints::EVALUATIONS)
            .headers(self.headers().await?)
            .send()
            .await?;

        // let val = serde_json::from_str(&res.text().await?)?;
        // Ok(val)
        Ok(res.text().await?)
    }

    /// get timetable
    pub async fn timetable(&self, from: DateTime, to: DateTime) -> AnyErr<Vec<Lesson>> {
        let client = reqwest::Client::new();
        let res = client
            .get(base(&self.school_id) + endpoints::TIMETABLE)
            .query(&[("datumTol", from.to_string()), ("datumIg", to.to_string())])
            .headers(self.headers().await?)
            .send()
            .await?;

        let val = serde_json::from_str(&res.text().await?)?;
        Ok(val)
    }

    /// get announced test
    pub async fn announced(&self, from: Option<DateTime>) -> AnyErr<String> {
        let query = if let Some(from) = from {
            vec![("datumTol", from.to_string())]
        } else {
            vec![]
        };
        let client = reqwest::Client::new();
        let res = client
            .get(base(&self.school_id) + endpoints::ANNOUNCED_TESTS)
            .query(&query)
            .headers(self.headers().await?)
            .send()
            .await?;

        // let val = serde_json::from_str(&res.text().await?)?;
        // Ok(val)
        Ok(res.text().await?)
    }

    /// get information about being absent
    pub async fn absencies(&self) -> AnyErr<String> {
        let client = reqwest::Client::new();
        let res = client
            .get(base(&self.school_id) + endpoints::ABSENCES)
            .headers(self.headers().await?)
            .send()
            .await?;

        // let val = serde_json::from_str(&res.text().await?)?;
        // Ok(val)
        Ok(res.text().await?)
    }
}
