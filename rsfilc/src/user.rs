use crate::{
    information::Info,
    messages::{Msg, MsgKind, MsgOview},
    timetable::next_lesson,
    token::Token,
    *,
};
use base64::{engine::general_purpose::STANDARD, Engine};
use chrono::{Days, Local, NaiveDate};
use ekreta::OptIrval;
use messages::NoteMsg;
use reqwest::{
    blocking::Client,
    header::{self, HeaderMap},
    redirect, Url,
};
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::{
    fmt::Debug,
    fs::{self, File, OpenOptions},
    io::{self, Write},
    time::Duration,
};

/// default timeout for api requests
const TIMEOUT: Duration = Duration::new(24, 0);

/// endpoint
pub const fn ep() -> &'static str {
    "/ellenorzo/V3/Sajat/TanuloAdatlap"
}

/// Kréta, app user
#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct User {
    /// the username, usually the `oktatási azonosító szám`: "7" + 10 numbers `7XXXXXXXXXX`
    username: String,
    /// the password, usually it defaults to the date of birth of the user: `YYYY-MM-DD`
    /// base64 encoded
    password: String,
    /// the id of the school the user goes to, usually looks like:  "klik" + 9 numbers: `klikXXXXXXXXX`
    school_id: String,
}
// basic stuff
impl User {
    /// get name of [`User`]
    ///
    /// # Errors
    ///
    /// net
    pub fn name(&self) -> Res<String> {
        Ok(self.fetch_info()?.name)
    }

    /// create new instance of [`User`]
    pub fn new(username: &str, password: &str, school_id: &str) -> Self {
        let password = STANDARD.encode(password);
        Self {
            username: username.to_string(),
            password: password.to_string(),
            school_id: school_id.to_string(),
        }
    }
    /// Returns the decoded password of this [`User`].
    ///
    /// # Panics
    ///
    /// Panics if decode fails.
    fn decode_password(&self) -> String {
        let decoded_password = STANDARD.decode(&self.password).unwrap();
        String::from_utf8(decoded_password).unwrap()
    }
    /// creates dummy [`User`], that won't be saved and shouldn't be used
    pub fn dummy() -> Self {
        info!("created dummy user");
        Self::new("", "", "")
    }

    /// create a [`User`] from cli and save it!
    ///
    /// # Panics
    ///
    /// `std::io::std(in/out)`
    pub fn create() -> Option<Self> {
        info!("creating user from cli");

        println!("please log in");
        print!("username: ");
        io::stdout().flush().unwrap();
        let mut username = String::new();
        io::stdin()
            .read_line(&mut username)
            .expect("couldn't read username");
        let username = username.trim();
        if username.is_empty() {
            println!("username is required");
            return None;
        }
        info!("recieved username {username} from cli");

        let password = rpassword::prompt_password("password: ").unwrap_or_default();
        if password.is_empty() {
            println!("password is required");
            return None;
        }
        info!("recieved password {password} from cli");

        print!("school_id: ");
        io::stdout().flush().unwrap();
        let mut school_id = String::new();
        io::stdin()
            .read_line(&mut school_id)
            .expect("couldn't read school_id");
        let school_id = school_id.trim();
        if school_id.is_empty() {
            println!("school_id is required");
            return None;
        }
        info!("recieved school_id {school_id} from cli");

        let user = Self::new(username, &password, school_id);
        if let Ok(name) = user.name() {
            println!("Hi {name}, nice to see you!");
        } else {
            println!("Sorry, couldn't authenticate, make sure you have internet connection and all your credentials are correct.");
            return None;
        }
        user.save();
        Some(user)
    }

    /// Load every saved [`User`] from [`cred_path()`]
    ///
    /// # Panics
    ///
    /// Panics if cred path does not exist.
    pub fn load_all() -> Vec<Self> {
        info!("loading users");
        let cred_path = cred_path().expect("couldn't find credential path");

        if !cred_path.exists() {
            warn!("credential path doesn't exist");
            return vec![];
        }

        let content = fs::read_to_string(cred_path).expect("couldn't read credentials.toml");
        // migth not be necessary
        if content.is_empty() {
            warn!("ain't no user credentials saved");
            return vec![];
        }

        let users: Users =
            toml::from_str(&content).expect("couldn't read user credentials from file");

        users.into()
    }
    /// save [`User`] credentials if not empty
    fn save(&self) {
        info!("saving user");
        if Self::load_all().contains(self) {
            warn!("{:?} is already saved, not saving", self);
            return;
        }
        let cred_path = cred_path().expect("couldn't find config dir");
        if !cred_path.exists() {
            info!("creating credential path");
            fs::create_dir_all(cred_path.parent().expect("couldn't get credential dir"))
                .expect("couldn't create credential dir");
        }
        let mut cred_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(cred_path)
            .expect("couldn't save user credentials");

        // don't save if a value is missing
        if self.username.is_empty() || self.password.is_empty() || self.school_id.is_empty() {
            warn!("user {:?} is missing data, not saving", self);
            return;
        }
        write!(
            cred_file,
            "{}",
            toml::to_string(&Users::from(vec![self.clone()])).expect("couldn't serialize user")
        )
        .expect("couldn't save user");
    }

    /// load [`User`] with [`User::username`] or [`User::name()`] from [`cred_path()`] and save it to [`config_path()`]
    pub fn load(username: &str) -> Option<Self> {
        info!("loading user with {username}");
        let mut matching_users = Vec::new();
        for user in Self::load_all() {
            if user
                .username
                .to_lowercase()
                .contains(&username.to_lowercase())
                || user
                    .name()
                    .is_ok_and(|nm| nm.to_lowercase().contains(&username.to_lowercase()))
            {
                matching_users.push(user);
            }
        }
        let user = matching_users.first()?;
        user.save_to_conf();

        Some(user.clone())
    }
    /// save [`User`] as default to config.toml
    ///
    /// # Panics
    ///
    /// - no config path
    /// - no parent dir
    /// - deser
    /// - writeln
    fn save_to_conf(&self) {
        info!("saving preferred user's name to config");
        let conf_path = config_path().expect("couldn't find config path");
        if !conf_path.exists() {
            fs::create_dir_all(conf_path.parent().expect("couldn't get config dir"))
                .expect("couldn't create config dir");
        }
        let mut conf_file = File::create(conf_path).expect("couldn't create config file");

        writeln!(
            conf_file,
            "{}",
            toml::to_string(&Config {
                default_username: self.username.clone()
            })
            .expect("couldn't deserialize user")
        )
        .expect("couldn't save user");
    }
    /// load `] configured in [`config_path()`]
    ///
    /// # Panics
    ///
    /// - can't read config content
    /// - invalid config
    pub fn load_conf() -> Option<Self> {
        info!("loading config");
        let conf_path = config_path()?;
        if !conf_path.exists() {
            return None;
        }
        let config_content = fs::read_to_string(conf_path).expect("couldn't read config file");
        let config = toml::from_str::<Config>(&config_content).expect("couldn't deser config");

        Self::load(&config.default_username)
    }

    /// print all lessons of a day
    pub fn print_day(&self, lessons: &[Lesson]) {
        if let Some(first_lesson) = lessons.first() {
            println!(
                "    {} ({})",
                &first_lesson.kezdet_idopont.pretty(),
                first_lesson.kezdet_idopont.hun_day_of_week()
            );
            if first_lesson.kamu_smafu() {
                let as_str = timetable::disp(first_lesson);
                println!("{as_str}");
                fill(&as_str, '|', None);
            }
            let todays_tests = self
                .fetch_all_announced((
                    Some(first_lesson.kezdet_idopont),
                    Some(lessons.last().unwrap().veg_idopont),
                ))
                .expect("couldn't fetch announced tests");
            let all_lessons_till_day = self
                .get_timetable(first_lesson.kezdet_idopont.date_naive(), true)
                .unwrap_or_default();
            // info!("all announced: {todays_tests:?}");

            // number of lessons at the same time
            let mut same_count = 0;

            for (i, lesson) in lessons.iter().filter(|l| !l.kamu_smafu()).enumerate() {
                // calculate `n`. this lesson is
                let n = if let Some(prev) = lessons.get((i as isize - 1) as usize) {
                    if prev.same_time(lesson) {
                        same_count += 1;
                    }
                    i + 1 - same_count
                } else {
                    i + 1 - same_count
                };
                // so fill_under() works fine
                let mut printer = format!("\n\n{n}. {}", timetable::disp(lesson));

                if let Some(test) = todays_tests
                    .iter()
                    .find(|ancd| ancd.orarendi_ora_oraszama.is_some_and(|x| x as usize == n))
                {
                    printer += &format!(
                        "\n| {}{}",
                        test.modja.leiras,
                        if let Some(topic) = test.temaja.as_ref() {
                            format!(": {}", topic)
                        } else {
                            "".into()
                        }
                    );
                }
                println!("{printer}");

                let (with, inlay_hint) = if lesson.happening() {
                    (
                        '$',
                        Some(format!(
                            "{} perc",
                            (lesson.veg_idopont - Local::now()).num_minutes()
                        )),
                    )
                } else if next_lesson(&all_lessons_till_day).is_some_and(|nxt| nxt == lesson) {
                    (
                        '>',
                        Some(format!(
                            "{} perc",
                            (lesson.kezdet_idopont - Local::now()).num_minutes()
                        )),
                    )
                } else if lesson.cancelled() {
                    (
                        'X',
                        Some(format!(
                            "elmarad{}",
                            if lesson.forecoming() { "" } else { "t" }
                        )),
                    )
                } else {
                    ('-', None)
                };
                fill(&printer, with, inlay_hint.as_deref());
            }
        }
    }
}

// interacting with API
impl User {
    /// base url of school with `school_id`
    /// <https://{school_id}.e-kreta.hu>
    fn base(&self) -> String {
        format!("https://{}.e-kreta.hu", self.school_id)
    }
    /// get headers which are necessary for making certain requests
    fn headers(&self) -> Res<HeaderMap> {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            format!("Bearer {}", self.fetch_token()?.access_token).parse()?,
        );
        headers.insert("User-Agent", endpoints::USER_AGENT.parse().unwrap());
        Ok(headers)
    }

    fn fetch_token(&self) -> Res<Token> {
        // Create a client with cookie store enable
        let client = Client::builder()
            .cookie_store(true)
            .redirect(redirect::Policy::none()) // this is needed so the client doesnt follow redirects by itself like a dumb little sheep
            .build()?;

        // initial login page
        let initial_url = "https://idp.e-kreta.hu/Account/Login?ReturnUrl=%2Fconnect%2Fauthorize%2Fcallback%3Fprompt%3Dlogin%26nonce%3DwylCrqT4oN6PPgQn2yQB0euKei9nJeZ6_ffJ-VpSKZU%26response_type%3Dcode%26code_challenge_method%3DS256%26scope%3Dopenid%2520email%2520offline_access%2520kreta-ellenorzo-webapi.public%2520kreta-eugyintezes-webapi.public%2520kreta-fileservice-webapi.public%2520kreta-mobile-global-webapi.public%2520kreta-dkt-webapi.public%2520kreta-ier-webapi.public%26code_challenge%3DHByZRRnPGb-Ko_wTI7ibIba1HQ6lor0ws4bcgReuYSQ%26redirect_uri%3Dhttps%253A%252F%252Fmobil.e-kreta.hu%252Fellenorzo-student%252Fprod%252Foauthredirect%26client_id%3Dkreta-ellenorzo-student-mobile-ios%26state%3Dkreten_student_mobile%26suppressed_prompt%3Dlogin";
        let response = client.get(initial_url).send()?;
        let raw_login_page_html = response.text()?;

        // Parse RVT token from HTML
        let login_page_html = Html::parse_document(&raw_login_page_html);
        let selector = Selector::parse("input[name='__RequestVerificationToken']")
            .map_err(|e| format!("Selector parse error: {}", e))?;

        let rvt = login_page_html
            .select(&selector)
            .next()
            .ok_or("RVT token not found in HTML")?
            .value()
            .attr("value")
            .ok_or("RVT token value missing")?; // shouldn't really ever happen but still

        // Perform login with credentials
        let decoded_password = self.decode_password();
        let login_url = "https://idp.e-kreta.hu/account/login";
        let form_data = [
        ("ReturnUrl", "/connect/authorize/callback?prompt=login&nonce=wylCrqT4oN6PPgQn2yQB0euKei9nJeZ6_ffJ-VpSKZU&response_type=code&code_challenge_method=S256&scope=openid%20email%20offline_access%20kreta-ellenorzo-webapi.public%20kreta-eugyintezes-webapi.public%20kreta-fileservice-webapi.public%20kreta-mobile-global-webapi.public%20kreta-dkt-webapi.public%20kreta-ier-webapi.public&code_challenge=HByZRRnPGb-Ko_wTI7ibIba1HQ6lor0ws4bcgReuYSQ&redirect_uri=https%3A%2F%2Fmobil.e-kreta.hu%2Fellenorzo-student%2Fprod%2Foauthredirect&client_id=kreta-ellenorzo-student-mobile-ios&state=kreten_student_mobile&suppressed_prompt=login"),
        ("IsTemporaryLogin", "False"),
        ("UserName", &self.username),
        ("Password", &decoded_password),
        ("InstituteCode", &self.school_id),
        ("loginType", "InstituteLogin"),
        ("__RequestVerificationToken", rvt),
    ];

        let response = client.post(login_url)
        .header(header::USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36")
        .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
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

        let response = client.get("https://idp.e-kreta.hu/connect/authorize/callback?prompt=login&nonce=wylCrqT4oN6PPgQn2yQB0euKei9nJeZ6_ffJ-VpSKZU&response_type=code&code_challenge_method=S256&scope=openid%20email%20offline_access%20kreta-ellenorzo-webapi.public%20kreta-eugyintezes-webapi.public%20kreta-fileservice-webapi.public%20kreta-mobile-global-webapi.public%20kreta-dkt-webapi.public%20kreta-ier-webapi.public&code_challenge=HByZRRnPGb-Ko_wTI7ibIba1HQ6lor0ws4bcgReuYSQ&redirect_uri=https%3A%2F%2Fmobil.e-kreta.hu%2Fellenorzo-student%2Fprod%2Foauthredirect&client_id=kreta-ellenorzo-student-mobile-ios&state=kreten_student_mobile&suppressed_prompt=login").send()?;

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
            ("client_id", "kreta-ellenorzo-student-mobile-ios"),
            ("grant_type", "authorization_code"),
        ];

        let response = client
            .post("https://idp.e-kreta.hu/connect/token")
            .form(&token_data)
            .send()?;

        let text = response.text()?;
        let mut logf = log_file("token")?;
        write!(logf, "{text}")?;

        let token = serde_json::from_str(&text)?;
        info!("received token");
        Ok(token)
    }

    /// get [`User`] info
    pub fn fetch_info(&self) -> Res<Info> {
        info!("recieved information about user");

        let txt = self.fetch(&(self.base() + user::ep()), "user_info", &[])?;

        let info = serde_json::from_str(&txt)?;
        Ok(info)
    }

    /// get all [`Eval`]s with `from` `to` or all
    ///
    /// # Panics
    ///
    /// sorting
    ///
    /// # Errors
    ///
    /// net
    pub fn fetch_evals(&self, mut interval: OptIrval) -> Res<Vec<Evaluation>> {
        let (cache_t, cache_content) = uncache("evals").unzip();
        let mut evals = if let Some(cached) = &cache_content {
            serde_json::from_str::<Vec<Evaluation>>(cached)?
        } else {
            vec![]
        };

        if let Some(ct) = cache_t {
            info!("from cached");
            interval.0 = Some(ct.to_day_with_hms());
        } else if let Some(from) = interval.0 {
            interval.0 = Some(from.to_day_with_hms());
        }
        if let Some(to) = interval.1 {
            interval.1 = Some(to.to_day_with_hms());
        }

        let mut fetch_err = false;
        let fetched_evals = self
            .fetch_from_endpoint("evals", interval)
            .inspect_err(|e| {
                fetch_err = true;
                warn!("couldn't fetch from E-Kréta server: {e:?}");
            });

        info!("recieved evals");

        evals.extend(fetched_evals.unwrap_or_default());
        evals.sort_by(|a, b| b.keszites_datuma.partial_cmp(&a.keszites_datuma).unwrap());
        evals.dedup();
        if interval.0.is_none() && !fetch_err {
            cache("evals", &serde_json::to_string(&evals)?)?;
        }
        Ok(evals)
    }

    pub fn get_timetable(&self, day: NaiveDate, everything_till_day: bool) -> Res<Vec<Lesson>> {
        let (_, cache_content) = uncache("timetable").unzip();
        let mut lessons = if let Some(cached) = &cache_content {
            serde_json::from_str::<Vec<Lesson>>(cached)?
        } else {
            vec![]
        };
        let day_from_mon = day
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_local_timezone(Local)
            .unwrap()
            .weekday()
            .number_from_monday()
            - 1;
        let day_till_sun = 7 - day_from_mon - 1;
        let week_start = day
            .checked_sub_days(Days::new(day_from_mon.into()))
            .unwrap();
        let week_end = day
            .checked_add_days(Days::new(day_till_sun.into()))
            .unwrap();

        let mon_start = week_start
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_local_timezone(Local)
            .unwrap();
        let sun_end = week_end
            .and_hms_opt(23, 59, 59)
            .unwrap()
            .and_local_timezone(Local)
            .unwrap();

        let mut fetch_err = false;
        let fetched_lessons_week = self
            .fetch_timetable(mon_start, sun_end)
            .inspect_err(|e| {
                fetch_err = true;
                warn!("couldn't deserialize data: {e:?}");
            })
            .unwrap_or_default();

        lessons.extend(fetched_lessons_week);
        lessons.sort_by(|a, b| a.kezdet_idopont.cmp(&b.kezdet_idopont));
        lessons.dedup();
        if !fetch_err {
            cache("timetable", &serde_json::to_string(&lessons)?)?;
        }
        if !everything_till_day {
            lessons.retain(|lsn| lsn.kezdet_idopont.date_naive() == day);
        }
        Ok(lessons)
    }

    /// get all [`Lesson`]s `from` `to` which makes up a timetable
    ///
    /// # Errors
    ///
    /// net
    ///
    /// # Panics
    ///
    /// - sorting
    fn fetch_timetable(&self, from: LDateTime, to: LDateTime) -> Res<Vec<ekreta::Lesson>> {
        let mut lessons: Vec<Lesson> = self.fetch_from_endpoint("timetable", (from, to))?;
        info!("recieved lessons");
        lessons.sort_by(|a, b| a.kezdet_idopont.partial_cmp(&b.kezdet_idopont).unwrap());
        Ok(lessons)
    }

    /// get [`Announced`] tests `from` `to` or all
    ///
    /// # Errors
    ///
    /// net
    ///
    /// # Panics
    ///
    /// sorting
    pub fn fetch_all_announced(&self, mut interval: OptIrval) -> Res<Vec<AnnouncedTest>> {
        let (cache_t, cache_content) = uncache("announced").unzip();
        let mut tests = if let Some(cached) = &cache_content {
            serde_json::from_str::<Vec<AnnouncedTest>>(cached)?
        } else {
            vec![]
        };

        if let Some(ct) = cache_t {
            info!("from cached");
            interval.0 = Some(ct.to_day_with_hms());
        } else if let Some(from) = interval.0 {
            interval.0 = Some(from.to_day_with_hms());
        }
        if let Some(to) = interval.1 {
            interval.1 = Some(to.to_day_with_hms());
        }

        let mut fetch_err = false;
        let fetched_tests = self
            .fetch_from_endpoint("announced", interval)
            .inspect_err(|e| {
                fetch_err = true;
                warn!("couldn't reach E-Kréta server: {e:?}");
            });

        tests.extend(fetched_tests.unwrap_or_default());
        tests.sort_by(|a, b| b.datum.partial_cmp(&a.datum).unwrap());
        tests.dedup();
        if let Some(from) = interval.0 {
            info!("filtering, from!");
            tests.retain(|ancd| ancd.datum.num_days_from_ce() >= from.num_days_from_ce());
        }
        if let Some(to) = interval.1 {
            info!("filtering, to!");
            tests.retain(|ancd| ancd.datum.num_days_from_ce() <= to.num_days_from_ce());
        }
        if interval.0.is_none() && !fetch_err {
            cache("announced", &serde_json::to_string(&tests)?)?;
        }

        Ok(tests)
    }

    /// get information about being [`Abs`]ent `from` `to` or all
    ///
    /// # Errors
    ///
    /// net
    ///
    /// # Panics
    ///
    /// sorting
    pub fn fetch_absences(&self, mut interval: OptIrval) -> Res<Vec<Absence>> {
        let (cache_t, cache_content) = uncache("absences").unzip();
        let mut absences = if let Some(cached) = &cache_content {
            serde_json::from_str::<Vec<Absence>>(cached)?
        } else {
            vec![]
        };

        if let Some(ct) = cache_t {
            info!("from cached");
            interval.0 = Some(ct.to_day_with_hms());
        } else if let Some(from) = interval.0 {
            interval.0 = Some(from.to_day_with_hms());
        }
        if let Some(to) = interval.1 {
            interval.1 = Some(to.to_day_with_hms());
        }

        let mut fetch_err = false;
        let fetched_absences = self
            .fetch_from_endpoint("absences", interval)
            .inspect_err(|e| {
                fetch_err = true;
                warn!("couldn't fetch from E-Kréta server: {e:?}")
            });

        info!("recieved absences");
        absences.extend(fetched_absences.unwrap_or_default());
        absences.sort_by(|a, b| b.ora.kezdo_datum.partial_cmp(&a.ora.kezdo_datum).unwrap());

        if interval.0.is_none() && !fetch_err {
            cache("absences", &serde_json::to_string(&absences)?)?;
        }

        Ok(absences)
    }

    /// get groups the [`User`] is a member of
    ///
    /// # Errors
    ///
    /// - net
    pub fn fetch_groups(&self) -> Res<String> {
        let txt = self.fetch(&(self.base() + endpoints::CLASSES), "groups", &[])?;
        // let all_announced = serde_json::from_str(&text)?;
        Ok(txt)
    }

    fn fetch_from_endpoint<E>(&self, log: &str, query: E::QueryInput) -> Res<Vec<E>>
    where
        E: ekreta::Endpoint + for<'a> Deserialize<'a>,
    {
        let uri = [self.base().as_str(), E::path()].concat();
        log::info!("sending request to {uri}");
        let query = E::query(&query)?;
        log::info!("query: {}", serde_json::to_string(&query).unwrap());
        let resp = Client::new()
            .get(uri)
            .query(&query)
            .headers(self.headers()?)
            .timeout(TIMEOUT);
        info!("sending request: {resp:?}");
        let resp = resp.send()?;
        let txt = resp.text()?;
        let mut logf = log_file(log).unwrap();
        write!(logf, "{txt}")?;
        // serde
        Ok(serde_json::from_str(&txt)?)
    }

    /// Fetch data from `url` with `query`, save log to [`log_file(`log`)`].
    fn fetch(&self, url: &str, log: &str, query: &[(&str, String)]) -> Res<String> {
        let client = Client::new();
        let res = client
            .get(url)
            .query(&query)
            .headers(self.headers()?)
            .timeout(TIMEOUT)
            .send()?;
        let text = res.text()?;

        let mut logf = log_file(log)?;
        write!(logf, "{text}")?;
        // cache(log, &text)?;
        // info!("cached.");

        Ok(text)
    }
}

/// [`Msg`]s and [`Attachment`]s
impl User {
    /// Download all [`Attachment`]s of this [`Msg`] to [`download_dir()`].
    ///
    /// # Errors
    /// - net
    pub fn download_attachments(&self, msg: &Msg) -> Res<()> {
        for am in msg.attachments() {
            info!("downloading file://{}", am.download_to().display());
            // don't download if already exists
            if am.download_to().exists() {
                info!("not downloading, already done");
                continue;
            }
            let mut f = File::create(am.download_to())?;

            let client = Client::new();
            client
                .get(endpoints::ADMIN.to_owned() + &endpoints::download_attachment(am.id))
                .headers(self.headers()?)
                .timeout(TIMEOUT)
                .send()?
                .copy_to(&mut f)?;

            info!("recieved file {}", &am.file_name);
        }
        Ok(())
    }

    /// get all [`MsgOview`]s of a [`MsgKind`]
    ///
    /// # Errors
    ///
    /// net
    pub fn fetch_msg_oviews_of_kind(&self, msg_kind: &MsgKind) -> Res<Vec<MsgOview>> {
        let txt = self.fetch(
            &(endpoints::ADMIN.to_owned() + &endpoints::get_all_msgs(&msg_kind.val())),
            "message_overviews",
            &[],
        )?;

        let msg = serde_json::from_str(&txt)?;
        info!("recieved message overviews of kind: {:?}", msg_kind);
        Ok(msg)
    }

    /// get up to `n` [`MsgOview`]s, of any [`MsgKind`]
    ///
    /// # Panics
    ///
    /// - sorting
    pub fn msg_oviews(&self, n: usize) -> Res<Vec<MsgOview>> {
        let mut msg_oviews = [
            self.fetch_msg_oviews_of_kind(&MsgKind::Recv)?,
            self.fetch_msg_oviews_of_kind(&MsgKind::Sent)?,
            self.fetch_msg_oviews_of_kind(&MsgKind::Del)?,
        ]
        .concat();

        msg_oviews.sort_by(|a, b| b.sent().partial_cmp(&a.sent()).unwrap());
        let max_n = msg_oviews.len();
        // don't exceed the lenght of msg_oviews
        let n = if n < max_n { n } else { max_n };
        let msg_oviews = msg_oviews.drain(0..n).collect();
        info!("recieved every message overview");
        Ok(msg_oviews)
    }

    /// Get whole [`Msg`] from the `id` of a [`MsgOview`]
    ///
    /// # Errors
    ///
    /// net
    pub fn fetch_full_msg(&self, msg_oview: &MsgOview) -> Res<Msg> {
        let txt = self.fetch(
            &(endpoints::ADMIN.to_owned() + &endpoints::get_msg(msg_oview.id)),
            "full_message",
            &[],
        )?;

        let msg = serde_json::from_str(&txt)?;
        info!("recieved full message: {:?}", msg);
        Ok(msg)
    }
    /// Fetch max `n` [`Msg`]s between `from` and `to`.
    /// Also download all `[Attachment]`s each [`Msg`] has.
    ///
    /// # Errors
    ///
    /// - net
    pub fn msgs(&self, interval: OptIrval) -> Res<Vec<Msg>> {
        let (cache_t, cache_content) = uncache("messages").unzip();
        let mut msgs = if let Some(cached) = &cache_content {
            serde_json::from_str::<Vec<Msg>>(cached)?
        } else {
            vec![]
        };
        let from = if let Some(ct) = cache_t {
            Some(ct)
        } else {
            interval.0
        };

        let mut fetched_msgs = Vec::new();
        let mut handles = Vec::new();

        let mut fetch_err = false;
        for msg_oview in self.msg_oviews(usize::MAX).unwrap_or_default() {
            // if isn't between `from`-`to`
            if from.is_some_and(|fm| msg_oview.sent() < fm)
                || interval.1.is_some_and(|to| msg_oview.sent() > to)
            {
                continue;
            }
            let s = self.clone();
            let h = std::thread::spawn(move || {
                s.fetch_full_msg(&msg_oview)
                    .inspect_err(|e| {
                        fetch_err = true;
                        warn!("couldn't fetch from E-Kréta server: {e:?}");
                    })
                    .unwrap()
            });
            handles.push(h);
        }
        let mut logf = log_file("messages")?;
        write!(logf, "{fetched_msgs:?}")?;

        let mut am_handles = Vec::new();

        for h in handles {
            fetched_msgs.push(h.join().unwrap());
        }

        msgs.extend(fetched_msgs);
        msgs.dedup();
        for msg in msgs.clone() {
            let s = self.clone();
            let xl = std::thread::spawn(move || {
                s.download_attachments(&msg)
                    .inspect_err(|e| warn!("couldn't fetch from E-Kréta server: {e:?}"))
                    .unwrap()
            });
            am_handles.push(xl);
        }
        for h in am_handles {
            let j = h.join();
            j.map_err(|e| *e.downcast::<String>().unwrap())?;
        }
        if interval.0.is_none() && !fetch_err {
            cache("messages", &serde_json::to_string(&msgs)?)?;
        }
        Ok(msgs)
    }

    /// get notes: additional messages the [`User`] recieved.
    ///
    /// # Errors
    ///
    /// - net
    pub fn fetch_note_msgs(&self, interval: OptIrval) -> Res<Vec<NoteMsg>> {
        let (cache_t, cache_content) = uncache("note_messages").unzip();
        let mut note_msgs = if let Some(cached) = &cache_content {
            serde_json::from_str::<Vec<NoteMsg>>(cached)?
        } else {
            vec![]
        };

        let mut query = vec![];
        if let Some(ct) = cache_t {
            info!("from cached");
            query.push(("datumTol", ct.make_kreta_valid()));
        } else if let Some(from) = interval.0 {
            query.push(("datumTol", from.make_kreta_valid()));
        }
        if let Some(to) = interval.1 {
            query.push(("datumIg", to.make_kreta_valid()));
        }

        let mut fetch_err = false;
        let txt = self
            .fetch(&(self.base() + endpoints::NOTES), "note_messages", &[])
            .inspect_err(|e| {
                fetch_err = true;
                warn!("couldn't reach E-Kréta server: {e:?}");
            })
            .unwrap_or_default();
        let fetched_note_msgs = serde_json::from_str::<Vec<NoteMsg>>(&txt).inspect_err(|e| {
            fetch_err = true;
            warn!("couldn't deserialize data: {e:?}");
        });

        note_msgs.extend(fetched_note_msgs.unwrap_or_default());
        if interval.0.is_none() && !fetch_err {
            cache("note_messages", &serde_json::to_string(&note_msgs)?)?;
        }

        Ok(note_msgs)
    }
}

/// Vec of [`User`]s, needed for deser
#[derive(PartialEq, Debug, Serialize, Deserialize)]
struct Users {
    users: Vec<User>,
}
impl From<Vec<User>> for Users {
    fn from(users: Vec<User>) -> Self {
        Users { users }
    }
}
impl From<Users> for Vec<User> {
    fn from(val: Users) -> Self {
        val.users
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
/// [`User`] preferences/config
struct Config {
    /// the default [`User`]s name to load
    default_username: String,
}

#[cfg(test)]
mod tests;
