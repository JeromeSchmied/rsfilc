#![allow(unused)]

use kreta::*;

/// Result from T and Box<dyn Error>
type AnyErr<T> = Result<T, Box<dyn std::error::Error>>;

mod kreta {
    pub fn base(school_id: &str) -> String {
        format!("https://{school_id}.e-kreta.hu")
    }

    const IDP: &str = "https://idp.e-kreta.hu";
    const ADMIN: &str = "https://eugyintezes.e-kreta.hu";
    const FILES: &str = "https://files.e-kreta.hu";
    pub const USER_AGENT: &str = "hu.ekreta.student/1.0.4/Android/0/0";

    /// ```json
    /// {
    ///   "GlobalMobileApiUrlDEV": "https://kretaglobalmobileapidev.ekreta.hu/",
    ///   "GlobalMobileApiUrlTEST": "https://kretaglobalmobileapitest.ekreta.hu",
    ///   "GlobalMobileApiUrlUAT": "https://kretaglobalmobileapiuat.ekreta.hu",
    ///   "GlobalMobileApiUrlPROD": "https://kretaglobalmobileapi2.ekreta.hu"
    /// }    
    /// ```
    #[derive(Deserialize, Serialize, Debug)]
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

    pub mod endpoints {
        pub const TOKEN: &str = "/connect/token";
        pub const NONCE: &str = "/nonce";
        pub const NOTES: &str = "/ellenorzo/V3/Sajat/Feljegyzesek";
        pub const EVENTS: &str = "/ellenorzo/V3/Sajat/FaliujsagElemek";
        pub const STUDENT: &str = "/ellenorzo/V3/Sajat/TanuloAdatlap";
        pub const EVALUATIONS: &str = "/ellenorzo/V3/Sajat/Ertekelesek";
        pub const ABSENCES: &str = "/ellenorzo/V3/Sajat/Mulasztasok";
        pub const GROUPS: &str = "/ellenorzo/V3/Sajat/OsztalyCsoportok";
        pub const CLASS_AVERAGES: &str = "/V3/Sajat/Ertekelesek/Atlagok/OsztalyAtlagok";
        pub const TIMETABLE: &str = "/ellenorzo/V3/Sajat/OrarendElemek";
        pub const ANNOUNCED_TESTS: &str = "/ellenorzo/V3/Sajat/BejelentettSzamonkeresek";
        pub const HOMEWORKS: &str = "/ellenorzo/V3/Sajat/HaziFeladatok";
        pub const HOMEWORK_DONE: &str = "/ellenorzo/V3/Sajat/HaziFeladatok/Megoldva";
        pub const CAPABILITIES: &str = "/ellenorzo/V3/Sajat/Intezmenyek";
    }

    pub mod admin_endpoints {
        const SEND_MESSAGE: &str = "/api/v1/kommunikacio/uzenetek";
        pub fn get_all_messages(endpoint: &str) -> String {
            format!("/api/v1/kommunikacio/postaladaelemek/{endpoint}")
        }
        pub fn get_message(id: &str) -> String {
            format!("/api/v1/kommunikacio/postaladaelemek/{id}")
        }
        const RECIPIENT_CATEGORIES: &str = "/api/v1/adatszotarak/cimzetttipusok";
        const AVAILABLE_CATEGORIES: &str = "/api/v1/kommunikacio/cimezhetotipusok";
        const RECIPIENTS_TEACHER: &str = "/api/v1/kreta/alkalmazottak/tanar";
        const UPLOAD_ATTACHMENT: &str = "/ideiglenesfajlok";
        fn download_attachment(id: &str) -> String {
            format!("/v1/dokumentumok/uzenetek/{id}")
        }
        const TRASH_MESSAGE: &str = "/api/v1/kommunikacio/postaladaelemek/kuka";
        const DELETE_MESSAGE: &str = "/api/v1/kommunikacio/postaladaelemek/torles";
    }

    #[derive(Deserialize, Serialize, Debug)]
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

        user_agent: String,
        client_id: String,
    }

    impl User {
        pub fn new(user_name: &str, password: &str, school_id: &str) -> Self {
            Self {
                user_name: user_name.to_string(),
                password: password.to_string(),
                school_id: school_id.to_string(),

                user_agent: USER_AGENT.to_string(),
                client_id: "kreta-ellenorzo-mobile-android".to_string(),
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
            let generated = base64::encode(result.into_bytes());

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

            let grant_type = String::from("password");
            let mut data = HashMap::new();
            data.insert("userName", &self.user_name);
            data.insert("password", &self.password);
            data.insert("institute_code", &self.school_id);
            data.insert("grant_type", &grant_type);
            data.insert("client_id", &self.client_id);

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
        pub async fn get_messages(&self, message_kind: MessageKind) -> AnyErr<Value> {
            let client = reqwest::Client::new();
            let res = client
                .get(base(&self.school_id) + &admin_endpoints::get_message(&message_kind.val()))
                .headers(self.get_headers().await)
                .send()
                .await?;

            let val = serde_json::from_str(&res.text().await?)?;
            Ok(val)
        }

        /// get evaluations
        pub async fn get_evals(&self) -> AnyErr<Value> {
            todo!()
        }
    }
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

    use crate::AnyErr;
    use hmac::{Hmac, Mac};
    use reqwest::header::HeaderMap;
    use serde::{Deserialize, Serialize};
    use serde_json::Value;
    use sha2::Sha512;
    use std::collections::HashMap;

    #[derive(Deserialize, Serialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct School {
        city: String,
        institute_code: String,
        name: String,
    }
    impl School {
        pub async fn get_from_refilc() -> AnyErr<Vec<School>> {
            let client = reqwest::Client::new();
            let res = client
                .get("https://api.refilc.hu/v1/public/school-list")
                .send()
                .await?;

            // let res = reqwest::get("https://api.refilc.hu/v1/public/school-list").await?;

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
            let apiruls = ApiUrls::get().await.unwrap();
        }
    }
}

#[tokio::main]
async fn main() -> AnyErr<()> {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 4 {
        println!("Usage: <username> <password> <school_id>");
        return Ok(());
    }

    let username = &args[1];
    let password = &args[2];
    let school_id = &args[3];
    let user = User::new(username, password, school_id);

    let schools = School::get_from_refilc().await?;
    println!("\ngot schools...");
    // println!("{:#?}", schools);

    let apiurls = ApiUrls::get().await?;
    println!("\ngot api urls...");
    // println!("{:#?}", apiurls);

    let access_token = user.get_token().await?;
    println!("\ngot access_token...");
    // println!("{:?}", access_token);

    let info = user.get_info().await?;
    println!("\ngot user info...");
    // println!("{:?}", info);

    // let messages = user.get_messages(MessageKind::Beerkezett).await?;
    // println!("\ngot messages...");
    // println!("{:?}", messages);

    Ok(())
}
