use crate::{
    information::Info,
    messages::{Msg, MsgKind, MsgOview},
    timetable::next_lesson,
    token::Token,
    *,
};
use base64::{engine::general_purpose::STANDARD, Engine};
use chrono::{DateTime, Local};
use hmac::{Hmac, Mac};
use reqwest::{
    blocking::{self, Client},
    header::HeaderMap,
};
use serde::{Deserialize, Serialize};
use sha2::Sha512;
use std::{
    collections::HashMap,
    fmt::Debug,
    fs::{self, File, OpenOptions},
    io::{self, Write},
    time::Duration,
};

use self::messages::NoteMsg;

/// default timeout for api requests
const TIMEOUT: Duration = Duration::new(24, 0);

/// `(from, to)` interval
pub type Interval = (Option<DateTime<Local>>, Option<DateTime<Local>>);

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
                &first_lesson.start().pretty(),
                first_lesson.start().hun_day_of_week()
            );
            if first_lesson.shite() {
                println!("{first_lesson}");
                fill(&first_lesson.to_string(), '|', None);
            }
            let todays_tests = self
                .fetch_all_announced((
                    Some(first_lesson.start()),
                    Some(lessons.last().unwrap().end()),
                ))
                .expect("couldn't fetch announced tests");
            info!("all announced: {todays_tests:?}");

            // number of lessons at the same time
            let mut same_count = 0;

            for (i, lesson) in lessons.iter().filter(|l| !l.shite()).enumerate() {
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
                let mut printer = format!("\n\n{n}. {lesson}");

                if let Some(test) = todays_tests
                    .iter()
                    .find(|ancd| ancd.nth.is_some_and(|x| x as usize == n))
                {
                    printer += &format!(
                        "\n| {}: {}",
                        test.kind(),
                        test.topic.clone().unwrap_or_default()
                    );
                }
                println!("{printer}");

                let (with, inlay_hint) = if lesson.happening() {
                    (
                        '$',
                        Some(format!(
                            "{} perc",
                            (lesson.end() - Local::now()).num_minutes()
                        )),
                    )
                } else if next_lesson(lessons).is_some_and(|nxt| nxt == lesson) {
                    (
                        '>',
                        Some(format!(
                            "{} perc",
                            (lesson.start() - Local::now()).num_minutes()
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

    /// get [`Token`] from credentials, [`User::school_id`]
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
    fn fetch_token(&self) -> Res<Token> {
        // Define the key as bytes
        let key: &[u8] = &[98, 97, 83, 115, 120, 79, 119, 108, 85, 49, 106, 77];

        // Get nonce
        let nonce = blocking::get([endpoints::IDP, endpoints::NONCE].concat())?.text()?;

        // Define the message
        let message = format!(
            "{}{nonce}{}",
            self.school_id.to_uppercase(),
            self.username.to_uppercase()
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
        headers.insert("User-Agent", endpoints::USER_AGENT.parse().unwrap());
        headers.insert("X-AuthorizationPolicy-Key", generated.parse().unwrap());
        headers.insert("X-AuthorizationPolicy-Version", "v2".parse().unwrap());
        headers.insert("X-AuthorizationPolicy-Nonce", nonce.parse().unwrap());

        let decoded_password = self.decode_password();

        let mut data = HashMap::new();
        data.insert("userName", self.username.as_str());
        data.insert("password", &decoded_password);
        data.insert("institute_code", &self.school_id);
        data.insert("grant_type", "password");
        data.insert("client_id", endpoints::CLIENT_ID);

        let client = Client::new();
        let res = client
            .post([endpoints::IDP, token::ep()].concat())
            .headers(headers)
            .form(&data)
            .timeout(TIMEOUT)
            .send()?;

        let text = res.text()?;
        let mut logf = log_file("token")?;
        write!(logf, "{text}")?;

        let token = serde_json::from_str(&text)?;
        info!("recieved token");
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
    pub fn fetch_evals(&self, interval: Interval) -> Res<Vec<Eval>> {
        let (cache_t, cache_content) = uncache("evals").unzip();
        let mut evals = if let Some(cached) = &cache_content {
            serde_json::from_str::<Vec<Eval>>(cached)?
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
            .fetch(&(self.base() + evals::ep()), "evals", &query)
            .inspect_err(|e| {
                fetch_err = true;
                warn!("couldn't fetch from E-Kréta server: {e:?}");
            });

        let fetched_evals = serde_json::from_str::<Vec<Eval>>(&txt.unwrap_or_default())
            .inspect_err(|e| {
                fetch_err = true;
                warn!("couldn't deserialize data: {e:?}")
            });
        info!("recieved evals");

        evals.extend(fetched_evals.unwrap_or_default());
        evals.sort_by(|a, b| b.earned().partial_cmp(&a.earned()).unwrap());
        evals.dedup();
        if interval.0.is_none() && !fetch_err {
            cache("evals", &serde_json::to_string(&evals)?)?;
        }
        Ok(evals)
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
    pub fn fetch_timetable(&self, from: DateTime<Local>, to: DateTime<Local>) -> Res<Vec<Lesson>> {
        let txt = self.fetch(
            &(self.base() + timetable::ep()),
            "timetable",
            &[("datumTol", from.to_string()), ("datumIg", to.to_string())],
        )?;

        let mut lessons = serde_json::from_str::<Vec<Lesson>>(&txt)?;
        info!("recieved lessons");
        lessons.sort_by(|a, b| a.start().partial_cmp(&b.start()).unwrap());
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
    pub fn fetch_all_announced(&self, interval: Interval) -> Res<Vec<Ancd>> {
        let (cache_t, cache_content) = uncache("announced").unzip();
        let mut tests = if let Some(cached) = &cache_content {
            serde_json::from_str::<Vec<Ancd>>(cached)?
        } else {
            vec![]
        };

        let mut query = vec![];
        if let Some(ct) = cache_t {
            info!("from cached");
            query.push(("datumTol", ct.make_kreta_valid()));
        } else if let Some(from) = interval.0 {
            info!("from date: {from:?}");
            query.push(("datumTol", from.make_kreta_valid()));
        };

        let mut fetch_err = false;
        let txt = self
            .fetch(&(self.base() + announced::ep()), "announced", &query)
            .inspect_err(|e| {
                fetch_err = true;
                warn!("couldn't reach E-Kréta server: {e:?}");
            });

        let fetched_tests = serde_json::from_str::<Vec<Ancd>>(&txt.unwrap_or_default())
            .inspect_err(|e| {
                fetch_err = true;
                warn!("couldn't deserialize data: {e:?}");
            });

        tests.extend(fetched_tests.unwrap_or_default());
        tests.sort_by(|a, b| b.day().partial_cmp(&a.day()).unwrap());
        tests.dedup();
        if let Some(from) = interval.0 {
            info!("filtering, from!");
            tests.retain(|ancd| ancd.day().num_days_from_ce() >= from.num_days_from_ce());
        }
        if let Some(to) = interval.1 {
            info!("filtering, to!");
            tests.retain(|ancd| ancd.day().num_days_from_ce() <= to.num_days_from_ce());
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
    pub fn fetch_absences(&self, interval: Interval) -> Res<Vec<Abs>> {
        let (cache_t, cache_content) = uncache("absences").unzip();
        let mut absences = if let Some(cached) = &cache_content {
            serde_json::from_str::<Vec<Abs>>(cached)?
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
            .fetch(&(self.base() + absences::ep()), "absences", &query)
            .inspect_err(|e| {
                fetch_err = true;
                warn!("couldn't fetch from E-Kréta server: {e:?}");
            });

        let fetched_absences = serde_json::from_str::<Vec<Abs>>(&txt.unwrap_or_default())
            .inspect_err(|e| {
                fetch_err = true;
                warn!("couldn't deserialize data: {e:?}");
            });
        info!("recieved absences");
        absences.extend(fetched_absences.unwrap_or_default());
        absences.sort_by(|a, b| b.start().partial_cmp(&a.start()).unwrap());

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
    pub fn msgs(&self, interval: Interval) -> Res<Vec<Msg>> {
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
    pub fn fetch_note_msgs(&self, interval: Interval) -> Res<Vec<NoteMsg>> {
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
