use crate::{
    absences::Abs,
    announced::Announced,
    config_path, cred_path,
    endpoints::{self, *},
    evals::Eval,
    info::Info,
    log_path,
    messages::{Msg, MsgKind, MsgOview},
    timetable::Lesson,
    token::Token,
    AnyErr,
};
use base64::{engine::general_purpose::STANDARD, Engine};
use chrono::{DateTime, Local, Utc};
use hmac::{Hmac, Mac};
use reqwest::header::HeaderMap;
use sha2::Sha512;
use std::{
    collections::HashMap,
    fs::{self, copy, File, OpenOptions},
    io::{self, Cursor, Write},
};

/// Kréta, app user
#[derive(Clone, PartialEq)]
pub struct User {
    /// the username, usually the `oktatási azonosító szám`: "7" + 10 numbers `7XXXXXXXXXX`
    username: String,
    /// the password, usually it defaults to the date of birth of the user: `YYYY-MM-DD`
    password: String,
    /// the id of the school the user goes to, usually looks like:  "klik" + 9 numbers: `klikXXXXXXXXX`
    school_id: String,
}
impl User {
    /// get name of [`User`]
    async fn name(&self) -> String {
        self.info().await.expect("couldn't get user info").nev
    }

    /// endpoint
    pub const fn ep() -> &'static str {
        "/ellenorzo/V3/Sajat/TanuloAdatlap"
    }

    /// create new instance of user and save it
    pub fn new(username: &str, password: &str, school_id: &str) -> Self {
        let user = Self {
            username: username.to_string(),
            password: password.to_string(),
            school_id: school_id.to_string(),
        };
        user.save();
        user
    }

    /// load user account from saved dir
    ///
    /// ```toml
    /// [[user]]
    /// username = "70123456789"
    /// password = "2000-01-01"
    /// school_id = "klik012345678"
    ///
    /// [[user]]
    /// username = "70000000000"
    /// password = "2002-01-01"
    /// school_id = "klik000000000"
    /// ```
    pub fn parse(content: &str) -> Option<Self> {
        let username = Self::get_val(content, "username");
        let password = Self::get_val(content, "password");
        let school_id = Self::get_val(content, "school_id");
        match (username, password, school_id) {
            (Some(un), Some(pw), Some(si)) => Some(User {
                username: un,
                password: pw,
                school_id: si,
            }),
            _ => None,
        }
    }

    /// get value for key from content (eg. toml file)
    fn get_val(content: &str, key: &str) -> Option<String> {
        let k = &format!("{key} = ");
        if !content.contains(k) {
            return None;
        }

        let val = content
            .lines()
            .filter(|line| !line.starts_with('#'))
            .find(|line| line.contains(k))?
            .split('=')
            .last()?
            .trim()
            .trim_matches(|c| c == '"' || c == '\'')
            .to_string();

        Some(val)
    }

    /// create a [`User`] from cli
    pub fn create() -> Self {
        println!("please login");
        print!("username: ");
        io::stdout().flush().unwrap();
        let mut username = String::new();
        io::stdin()
            .read_line(&mut username)
            .expect("couldn't read username");

        print!("password: ");
        io::stdout().flush().unwrap();
        let mut password = String::new();
        io::stdin()
            .read_line(&mut password)
            .expect("couldn't read password");

        print!("school_id: ");
        io::stdout().flush().unwrap();
        let mut school_id = String::new();
        io::stdin()
            .read_line(&mut school_id)
            .expect("couldn't read school_id");

        Self::new(username.trim(), password.trim(), school_id.trim())
    }

    /// Load every saved [`User`] from [`User::cred_path`]
    ///
    /// # Panics
    ///
    /// Panics if cred path does not exist.
    pub fn load_all() -> Vec<Self> {
        let cred_path = cred_path().expect("couldn't find config dir");

        if !cred_path.exists() {
            return vec![];
        }

        let content = fs::read_to_string(cred_path).expect("couldn't read credentials from file");

        let mut users = Vec::new();
        for user_s in content.split("[[user]]") {
            if let Some(parsed_user) = Self::parse(user_s) {
                users.push(parsed_user);
            }
        }

        users
    }
    /// save user credentials
    fn save(&self) {
        if Self::load_all().contains(self) {
            return;
        }
        let cred_path = cred_path().expect("couldn't find config dir");
        if !cred_path.exists() {
            fs::create_dir_all(cred_path.parent().expect("couldn't get config dir"))
                .expect("couldn't create config dir");
        }
        let mut cred_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(cred_path)
            .expect("couldn't save user credentials");

        // don't save if some value is missing
        if self.username.is_empty() || self.password.is_empty() || self.school_id.is_empty() {
            return;
        }

        writeln!(cred_file, "[[user]]").unwrap();
        writeln!(cred_file, "username = \"{}\"", self.username).unwrap();
        writeln!(cred_file, "password = \"{}\"", self.password).unwrap();
        writeln!(cred_file, "school_id = \"{}\"", self.school_id).unwrap();
    }
    /// greet user
    pub async fn greet(&self) {
        if let Ok(info) = self.info().await {
            println!("Hello {}!\n\n", info.nev);
        }
    }
    /// load [`User`] with [`User::username`] and save it to [`User::config_path`]
    pub async fn load_user(username: &str) -> Option<Self> {
        let mut matching_users = Vec::new();
        for user in Self::load_all() {
            if user
                .name()
                .await
                .to_lowercase()
                .contains(&username.to_lowercase())
            {
                matching_users.push(user);
            }
        }
        let user = matching_users.first()?;

        user.save_to_conf().await;

        Some(user.clone())
    }
    /// save [`User`] as default to config.toml
    async fn save_to_conf(&self) {
        let conf_path = config_path().expect("couldn't find config path");
        if !conf_path.exists() {
            fs::create_dir_all(conf_path.parent().expect("couldn't get config dir"))
                .expect("couldn't create config dir");
        }
        let mut conf_file = File::create(conf_path).expect("couldn't create config file");

        writeln!(conf_file, "[user]").unwrap();
        writeln!(conf_file, "name = \"{}\"", self.name().await).unwrap();
    }
    /// load [`User`] configured in config.toml
    pub async fn load_conf() -> Option<Self> {
        let conf_path = config_path().expect("couldn't find config path");
        if !conf_path.exists() {
            return None;
        }
        let config_content = fs::read_to_string(conf_path).expect("couldn't read config file");
        let username = Self::get_val(&config_content, "name")?;

        Self::load_user(&username).await
    }

    /// get headers which are necessary for making certain requests
    async fn headers(&self) -> AnyErr<HeaderMap> {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            format!("Bearer {}", self.token().await?.access_token).parse()?,
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
    async fn token(&self) -> AnyErr<Token> {
        // Define the key as bytes
        let key: &[u8] = &[98, 97, 83, 115, 120, 79, 119, 108, 85, 49, 106, 77];

        // Get nonce
        let nonce = reqwest::blocking::get([endpoints::IDP, endpoints::NONCE].concat())
            .await?
            .text()
            .await?;

        // Define the message as bytes
        let message = format!(
            "{}{}{}",
            self.school_id.to_uppercase(),
            nonce,
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

        let mut data = HashMap::new();
        data.insert("userName", self.username.as_str());
        data.insert("password", &self.password);
        data.insert("institute_code", &self.school_id);
        data.insert("grant_type", "password");
        data.insert("client_id", endpoints::CLIENT_ID);

        let client = reqwest::Client::new();
        let res = client
            .post([endpoints::IDP, Token::ep()].concat())
            .headers(headers)
            .form(&data)
            .send()
            .await?;

        let text = res.text().await?;
        let mut logf = File::create(log_path("token"))?;
        write!(logf, "{text}")?;

        let token = serde_json::from_str(&text)?;
        Ok(token)
    }

    /// Returns the current [`Lesson`](s) of this [`User`] if any.
    pub async fn current_lesson(&self) -> Option<Vec<Lesson>> {
        let now = Local::now();
        let current = self.timetable(now, now).await.ok()?;

        if current.is_empty() {
            None
        } else {
            Some(current)
        }
    }

    /// get [`User`] info
    pub async fn info(&self) -> AnyErr<Info> {
        let client = reqwest::Client::new();
        let res = client
            .get(base(&self.school_id) + User::ep())
            .headers(self.headers().await?)
            .send()
            .await?;

        let text = res.text().await?;
        let mut logf = File::create(log_path("info"))?;
        write!(logf, "{text}")?;

        let info = serde_json::from_str(&text)?;
        Ok(info)
    }

    /// get all [`MsgOview`]s of a [`MsgKind`]
    pub async fn msg_oviews_of_kind(&self, msg_kind: MsgKind) -> AnyErr<Vec<MsgOview>> {
        let client = reqwest::Client::new();
        let res = client
            .get(endpoints::ADMIN.to_owned() + &endpoints::get_all_msgs(&msg_kind.val()))
            .headers(self.headers().await?)
            .send()
            .await?;

        let text = res.text().await?;
        let mut logf = File::create(log_path("messages"))?;
        write!(logf, "{text}")?;

        let msg = serde_json::from_str(&text)?;
        Ok(msg)
    }

    /// get all messages, of any kind
    pub async fn all_msg_oviews(&self) -> AnyErr<Vec<MsgOview>> {
        let mut msgs = Vec::new();

        msgs = [msgs, self.msg_oviews_of_kind(MsgKind::Recv).await?].concat();
        msgs = [msgs, self.msg_oviews_of_kind(MsgKind::Sent).await?].concat();
        msgs = [msgs, self.msg_oviews_of_kind(MsgKind::Del).await?].concat();

        Ok(msgs)
    }

    /// Get whole [`Msg`] from the `id` of a [`MsgOview`]
    pub async fn full_msg(&self, msg_oview: &MsgOview) -> AnyErr<Msg> {
        let client = reqwest::Client::new();
        let res = client
            .get(endpoints::ADMIN.to_owned() + &endpoints::get_msg(msg_oview.azonosito))
            .headers(self.headers().await?)
            .send()
            .await?;

        let text = res.text().await?;
        let mut logf = File::create(log_path("full_message"))?;
        write!(logf, "{text}")?;

        let msg = serde_json::from_str(&text)?;
        Ok(msg)
    }

    /// get all [`Eval`]s with `from` `to` or all
    pub async fn evals(
        &self,
        from: Option<DateTime<Local>>,
        to: Option<DateTime<Local>>,
    ) -> AnyErr<Vec<Eval>> {
        let mut query = vec![];
        if let Some(from) = from {
            query.push(("datumTol", from.to_rfc3339()));
        }
        if let Some(to) = to {
            query.push(("datumIg", to.to_rfc3339()));
        }
        let client = reqwest::Client::new();
        let res = client
            .get(base(&self.school_id) + Eval::ep())
            .query(&query)
            .headers(self.headers().await?)
            .send()
            .await?;

        let text = res.text().await?;
        let mut logf = File::create(log_path("evals"))?;
        write!(logf, "{text}")?;

        let evals = serde_json::from_str(&text)?;
        Ok(evals)
    }

    /// get all [`Lesson`]s `from` `to` which makes up a timetable
    pub async fn timetable(
        &self,
        from: DateTime<Local>,
        to: DateTime<Local>,
    ) -> AnyErr<Vec<Lesson>> {
        let client = reqwest::Client::new();
        let res = client
            .get(base(&self.school_id) + Lesson::ep())
            .query(&[("datumTol", from.to_string()), ("datumIg", to.to_string())])
            .headers(self.headers().await?)
            .send()
            .await?;
        let text = res.text().await?;

        let mut logf = File::create(log_path("timetable"))?;
        write!(logf, "{text}")?;

        let mut lessons = serde_json::from_str::<Vec<Lesson>>(&text)?;
        lessons.sort_by(|a, b| a.start().partial_cmp(&b.start()).expect("couldn't compare"));
        Ok(lessons)
    }

    /// get [`Announced`] tests `from` or all
    pub async fn all_announced(&self, from: Option<DateTime<Utc>>) -> AnyErr<Vec<Announced>> {
        let query = if let Some(from) = from {
            vec![("datumTol", from.to_rfc3339())]
        } else {
            vec![]
        };
        let client = reqwest::Client::new();
        let res = client
            .get(base(&self.school_id) + Announced::ep())
            .query(&query)
            .headers(self.headers().await?)
            .send()
            .await?;

        let text = res.text().await?;
        let mut logf = File::create(log_path("announced"))?;
        write!(logf, "{text}")?;

        let all_announced = serde_json::from_str(&text)?;
        Ok(all_announced)
    }

    pub async fn download_attachments(&self, msg: &Msg) -> AnyErr<()> {
        // let download_dir = dirs::download_dir().expect("couldn't find Downloads");
        for am in msg.attachments() {
            eprintln!("downloading {}", am.file_name);

            let client = reqwest::Client::new();
            let res = client
                .get(endpoints::ADMIN.to_owned() + &endpoints::download_attachment(am.id))
                .headers(self.headers().await?)
                .send()
                .await?;
            eprintln!("{}", endpoints::download_attachment(am.id));
            // let text = res.text().await?;
            let mut f = File::create(am.file_name).expect("wronk filepath!");
            let mut content = Cursor::new(res.bytes().await?);
            // f.write_all(&content)?;
            // copy(&mut content, &mut f);

            // let bytes = res.bytes().await?;
            // f.write_all(&bytes).expect("wronk attachment!");

            // println!("{}", text);
        }
        Ok(())
    }

    /// get information about being [`Abs`]ent `from` `to` or all
    pub async fn absences(
        &self,
        from: Option<DateTime<Local>>,
        to: Option<DateTime<Local>>,
    ) -> AnyErr<Vec<Abs>> {
        let mut query = vec![];
        if let Some(from) = from {
            query.push(("datumTol", from.to_rfc3339()));
        }
        if let Some(to) = to {
            query.push(("datumIg", to.to_rfc3339()));
        }
        let client = reqwest::Client::new();
        let res = client
            .get(base(&self.school_id) + Abs::ep())
            .query(&query)
            .headers(self.headers().await?)
            .send()
            .await?;

        let text = res.text().await?;
        let mut logf = File::create(log_path("absences"))?;
        write!(logf, "{text}")?;

        let abss = serde_json::from_str(&text)?;
        Ok(abss)
    }

    /// get groups the [`User`] is a member of
    pub async fn groups(&self) -> AnyErr<String> {
        let client = reqwest::Client::new();
        let res = client
            .get(base(&self.school_id) + endpoints::CLASSES)
            .headers(self.headers().await?)
            .send()
            .await?;

        let text = res.text().await?;
        let mut logf = File::create(log_path("groups"))?;
        write!(logf, "{text}")?;

        // let all_announced = serde_json::from_str(&text)?;
        Ok(text)
    }
}
