//! these URLs are for accessing main API I guess

use crate::AnyErr;
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
    pub fn api_url() -> String {
        "https://kretamobile.blob.core.windows.net/configuration/ConfigurationDescriptor.json"
            .to_string()
    }
    pub async fn api_urls() -> AnyErr<ApiUrls> {
        let res = reqwest::get(ApiUrls::api_url()).await?;

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
        let apiurls = ApiUrls::api_urls().await;
        assert!(apiurls.is_ok());
    }
}
