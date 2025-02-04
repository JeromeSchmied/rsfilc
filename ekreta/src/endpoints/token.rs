//! Bearer token, that's used to access any user-related data

use super::Endpoint;
use crate::Res;
use serde::Deserialize;

/// Token
///
/// consists of
/// - `access_token`
/// - `refresh_token`
/// - extra stuff, not needed
#[derive(Deserialize, Debug)]
pub struct Token {
    pub id_token: String,
    /// the bearer token
    pub access_token: String,
    pub expires_in: u32,
    pub token_type: String,
    /// the token which can be used to refresh the bearer token
    pub refresh_token: String,
    pub scope: String,
}
impl Endpoint for Token {
    /// username, password, institute-code
    type Args = (String, String, String, String);

    fn path(_args: &Self::Args) -> String {
        "/connect/token".into()
    }
    fn base_url(_args: impl AsRef<str>) -> String {
        super::base::IDP.into()
    }
    fn query(input: &Self::Args) -> Res<impl serde::Serialize> {
        Ok(vec![
            ("ReturnUrl", "/connect/authorize/callback?prompt=login&nonce=wylCrqT4oN6PPgQn2yQB0euKei9nJeZ6_ffJ-VpSKZU&response_type=code&code_challenge_method=S256&scope=openid%20email%20offline_access%20kreta-ellenorzo-webapi.public%20kreta-eugyintezes-webapi.public%20kreta-fileservice-webapi.public%20kreta-mobile-global-webapi.public%20kreta-dkt-webapi.public%20kreta-ier-webapi.public&code_challenge=HByZRRnPGb-Ko_wTI7ibIba1HQ6lor0ws4bcgReuYSQ&redirect_uri=https%3A%2F%2Fmobil.e-kreta.hu%2Fellenorzo-student%2Fprod%2Foauthredirect&client_id=kreta-ellenorzo-student-mobile-ios&state=kreten_student_mobile&suppressed_prompt=login"),
            ("UserName", &input.0),
            ("Password", &input.1),
            ("InstituteCode", &input.2),
            ("IsTemporaryLogin", "False"),
            ("loginType", "InstituteLogin"),
            ("__RequestVerificationToken", &input.3),
        ])
    }
    fn method() -> http::Method {
        http::Method::POST
    }
    fn headers(_input: &impl serde::Serialize) -> Res<Option<http::HeaderMap>> {
        let hm = http::HeaderMap::from_iter([
                (http::header::USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36".parse()?),
                (http::header::CONTENT_TYPE, "application/x-www-form-urlencoded".parse()?)
            ]).to_owned();

        Ok(Some(hm))
    }
}
