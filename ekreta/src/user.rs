use crate::consts::{CLIENT_ID, TIMEOUT};
use crate::*;
use http::{header, HeaderMap};
use reqwest::blocking::{Client, Response};
use reqwest::Url;
use serde::{Deserialize, Serialize};
use std::{fs::File, path::PathBuf};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct User {
    /// the username, usually the `oktatási azonosító szám`: "7" + 10 numbers `7XXXXXXXXXX`
    pub username: String,
    /// the password, usually it defaults to the date of birth of the user: `YYYY-MM-DD`
    pub password: String,
    /// the id of the school the user goes to, usually looks like:  "klik" + 9 numbers: `klikXXXXXXXXX`
    pub schoolid: String,
}
// fetch_.*
impl User {
    pub fn fetch_info(&self, headers: &HeaderMap) -> Res<UserInfo> {
        self.fetch_single::<UserInfo, UserInfo>((), headers)
    }
    pub fn fetch_evals(&self, interval: OptIrval, headers: &HeaderMap) -> Res<Vec<Evaluation>> {
        self.fetch_vec(interval, headers)
    }
    pub fn fetch_timetable(
        &self,
        interval: (LDateTime, LDateTime),
        headers: &HeaderMap,
    ) -> Res<Vec<Lesson>> {
        self.fetch_vec(interval, headers)
    }
    pub fn fetch_absences(&self, interval: OptIrval, headers: &HeaderMap) -> Res<Vec<Absence>> {
        self.fetch_vec(interval, headers)
    }
    pub fn fetch_classes(&self, headers: &HeaderMap) -> Res<Vec<Class>> {
        self.fetch_vec((), headers)
    }
    pub fn fetch_announced_tests(
        &self,
        interval: OptIrval,
        headers: &HeaderMap,
    ) -> Res<Vec<AnnouncedTest>> {
        self.fetch_vec(interval, headers)
    }
}

// messages
impl User {
    pub fn fetch_full_msg(
        &self,
        msg_oview: Option<&MessageOverview>,
        headers: &HeaderMap,
    ) -> Res<MessageItem> {
        let id = msg_oview.map(|mov| mov.azonosito);
        self.fetch_single::<MessageItem, MessageItem>(id, headers)
    }
    pub fn fetch_note_msgs(
        &self,
        interval: OptIrval,
        headers: &HeaderMap,
    ) -> Res<Vec<NoteMessage>> {
        self.fetch_vec(interval, headers)
    }
    pub fn fetch_msg_oview_of_kind(
        &self,
        msg_kind: MessageKind,
        headers: &HeaderMap,
    ) -> Res<Vec<MessageOverview>> {
        self.fetch_vec(msg_kind, headers)
    }
    pub fn fetch_msg_oviews(&self, headers: &HeaderMap) -> Res<Vec<MessageOverview>> {
        Ok([
            self.fetch_msg_oview_of_kind(MessageKind::Recv, headers)?,
            self.fetch_msg_oview_of_kind(MessageKind::Sent, headers)?,
            self.fetch_msg_oview_of_kind(MessageKind::Del, headers)?,
        ]
        .concat())
    }
    pub fn download_attachment_to(
        &self,
        id: u32,
        out_path: PathBuf,
        headers: &HeaderMap,
    ) -> Res<()> {
        let mut f = File::create(out_path)?;
        let mut resp = self.get_response::<Attachment>(id, headers)?;
        resp.copy_to(&mut f)?;
        Ok(())
    }
}
// token
impl User {
    pub fn refresh_token_resp(schoolid: &str, refresh_token: &str) -> Res<Response> {
        let response = reqwest::blocking::Client::new()
            .post("https://idp.e-kreta.hu/connect/token")
            .form(&[
                ("institute_code", schoolid),
                ("grant_type", "refresh_token"),
                ("client_id", consts::CLIENT_ID),
                ("refresh_token", refresh_token),
            ])
            .header(header::USER_AGENT, consts::USER_AGENT)
            .send()?;
        Ok(response)
    }
    pub fn refresh_token(&self, refresh_token: &str) -> Res<Token> {
        let text = Self::refresh_token_resp(&self.schoolid, refresh_token)?.text()?;
        Ok(serde_json::from_str(&text)?)
    }
    pub fn get_token_resp(&self) -> Res<Response> {
        // Create a client with cookie store enable
        let client = Client::builder()
            .cookie_store(true)
            .redirect(reqwest::redirect::Policy::none()) // this is needed so the client doesnt follow redirects by itself like a dumb little sheep
            .build()?;

        // initial login page
        let initial_url = format!("https://idp.e-kreta.hu/Account/Login?ReturnUrl=%2Fconnect%2Fauthorize%2Fcallback%3Fprompt%3Dlogin%26nonce%3DwylCrqT4oN6PPgQn2yQB0euKei9nJeZ6_ffJ-VpSKZU%26response_type%3Dcode%26code_challenge_method%3DS256%26scope%3Dopenid%2520email%2520offline_access%2520kreta-ellenorzo-webapi.public%2520kreta-eugyintezes-webapi.public%2520kreta-fileservice-webapi.public%2520kreta-mobile-global-webapi.public%2520kreta-dkt-webapi.public%2520kreta-ier-webapi.public%26code_challenge%3DHByZRRnPGb-Ko_wTI7ibIba1HQ6lor0ws4bcgReuYSQ%26redirect_uri%3Dhttps%253A%252F%252Fmobil.e-kreta.hu%252Fellenorzo-student%252Fprod%252Foauthredirect%26client_id%3D{CLIENT_ID}%26state%3Dkreten_student_mobile%26suppressed_prompt%3Dlogin");
        let response = client.get(initial_url).send()?;
        let raw_login_page_html = response.text()?;

        // Parse RVT token from HTML
        let login_page_html = scraper::Html::parse_document(&raw_login_page_html);
        let selector = scraper::Selector::parse("input[name='__RequestVerificationToken']")
            .map_err(|e| format!("Selector parse error: {e}"))?;

        let rvt = login_page_html
            .select(&selector)
            .next()
            .ok_or("RVT token not found in HTML")?
            .value()
            .attr("value")
            .ok_or("RVT token value missing")?; // shouldn't really ever happen but still

        // Perform login with credentials
        let login_url = "https://idp.e-kreta.hu/account/login";
        let query_data = (
            self.username.clone(),
            self.password.clone(),
            self.schoolid.clone(),
            rvt.to_string(),
        );
        // it's called query, but that doesn't matter
        let form_data = Token::query(&query_data)?;

        let headers = Token::headers(&"")?.unwrap();
        let response = client
            .post(login_url)
            .headers(headers)
            .form(&form_data)
            .send()?;

        // Check if the response status is 200 (OK)
        if !response.status().is_success() {
            return Err(format!(
                "Login failed: check your credentials. Status: {}",
                response.status()
            )
            .into());
        }

        let response = client.get(format!("https://idp.e-kreta.hu/connect/authorize/callback?prompt=login&nonce=wylCrqT4oN6PPgQn2yQB0euKei9nJeZ6_ffJ-VpSKZU&response_type=code&code_challenge_method=S256&scope=openid%20email%20offline_access%20kreta-ellenorzo-webapi.public%20kreta-eugyintezes-webapi.public%20kreta-fileservice-webapi.public%20kreta-mobile-global-webapi.public%20kreta-dkt-webapi.public%20kreta-ier-webapi.public&code_challenge=HByZRRnPGb-Ko_wTI7ibIba1HQ6lor0ws4bcgReuYSQ&redirect_uri=https%3A%2F%2Fmobil.e-kreta.hu%2Fellenorzo-student%2Fprod%2Foauthredirect&client_id={CLIENT_ID}&state=kreten_student_mobile&suppressed_prompt=login")).send()?;

        // Follow the redirect manually to get the code
        let location = response
            .headers()
            .get(header::LOCATION)
            .ok_or("No Location header after login redirect")?
            .to_str()?;

        // Extract code from the location header
        let code = Url::parse(location)?
            .query_pairs()
            .find(|(k, _)| k == "code")
            .map(|(_, v)| v.into_owned())
            .ok_or("Authorization code not found")?; // this also shouldn't ever happen probably

        // Exchange code for access token
        let token_data = [
            ("code", code.as_str()),
            (
                "code_verifier",
                "DSpuqj_HhDX4wzQIbtn8lr8NLE5wEi1iVLMtMK0jY6c",
            ),
            (
                "redirect_uri",
                "https://mobil.e-kreta.hu/ellenorzo-student/prod/oauthredirect",
            ),
            ("client_id", CLIENT_ID),
            ("grant_type", "authorization_code"),
        ];

        let token_url = [Token::base_url(""), Token::path(&query_data)].concat();
        let response = client.post(token_url).form(&token_data).send()?;

        Ok(response)
    }
    pub fn fetch_token(&self) -> Res<Token> {
        let text = self.get_token_resp()?.text()?;
        Ok(serde_json::from_str(&text)?)
    }
}
impl User {
    // /// get headers which are necessary for making certain requests
    // pub fn headers(&self) -> Res<HeaderMap> {
    //     Ok(HeaderMap::from_iter([
    //         (
    //             header::AUTHORIZATION,
    //             format!("Bearer {}", self.fetch_token()?.access_token).parse()?,
    //         ),
    //         (header::USER_AGENT, consts::USER_AGENT.parse()?),
    //     ]))
    // }

    pub fn get_response<E>(&self, query: E::Args, headers: &HeaderMap) -> Res<Response>
    where
        E: crate::Endpoint + for<'a> Deserialize<'a>,
    {
        let base = E::base_url(&self.schoolid);
        let uri = [base, E::path(&query)].concat();
        let query = E::query(&query)?;
        let resp = Client::new()
            .get(uri)
            .query(&query)
            .headers(headers.clone())
            .timeout(TIMEOUT);
        Ok(resp.send()?)
    }
    pub fn fetch_single<E, D>(&self, query: E::Args, headers: &HeaderMap) -> Res<D>
    where
        E: crate::Endpoint + for<'a> Deserialize<'a>,
        D: for<'a> Deserialize<'a>,
    {
        let resp = self.get_response::<E>(query, headers)?;
        let txt = resp.text()?;
        Ok(serde_json::from_str(&txt)?)
    }

    pub fn fetch_vec<E>(&self, query: E::Args, headers: &HeaderMap) -> Res<Vec<E>>
    where
        E: crate::Endpoint + for<'a> Deserialize<'a>,
    {
        self.fetch_single::<E, Vec<E>>(query, headers)
    }
}
