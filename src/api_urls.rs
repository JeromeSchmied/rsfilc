//! these URLs are for accessing main API I guess

#![allow(unused)]

use crate::Res;
use serde::Deserialize;

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
    pub fn api_url() -> &'static str {
        "https://kretamobile.blob.core.windows.net/configuration/ConfigurationDescriptor.json"
    }
    pub fn get() -> Res<ApiUrls> {
        let res = reqwest::blocking::get(ApiUrls::api_url())?;

        Ok(serde_json::from_str(&res.text()?)?)
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

    #[ignore]
    fn api_url_get() {
        let apiurls = ApiUrls::get();
        assert!(apiurls.is_ok());
    }
}
