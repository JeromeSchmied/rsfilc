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
use reqwest::{
    blocking::{self, Client},
    header::HeaderMap,
};
use serde::{Deserialize, Serialize};
use sha2::Sha512;
use std::{
    collections::HashMap,
    fs::{self, File, OpenOptions},
    io::{self, Write},
};

/// Kréta, app user
#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
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
    pub fn name(&self) -> String {
        self.info().expect("couldn't get user info").nev
    }

    /// endpoint
    pub const fn ep() -> &'static str {
        "/ellenorzo/V3/Sajat/TanuloAdatlap"
    }

    /// create new instance of [`User`]
    pub fn new(username: &str, password: &str, school_id: &str) -> Self {
        Self {
            username: username.to_string(),
            password: password.to_string(),
            school_id: school_id.to_string(),
        }
    }

    /// create a [`User`] from cli and save it!
    pub fn create() -> Self {
        println!("please log in");
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

        let user = Self::new(username.trim(), password.trim(), school_id.trim());
        user.save();
        user
    }

    /// Load every saved [`User`] from [`cred_path()`]
    ///
    /// # Panics
    ///
    /// Panics if cred path does not exist.
    pub fn load_all() -> Vec<Self> {
        let cred_path = cred_path().expect("couldn't find credential path");

        if !cred_path.exists() {
            return vec![];
        }

        let content = fs::read_to_string(cred_path).expect("couldn't read credentials.toml");
        // migth not be necessary
        if content.is_empty() {
            return vec![];
        }

        let users: Users =
            toml::from_str(&content).expect("couldn't read user credentials from file");
        // eprintln!("{:?}", users);

        users.into()
    }
    /// save [`User`] credentials if not empty
    fn save(&self) {
        // eprintln!("saving user...");
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

        // don't save if a value is missing
        if self.username.is_empty() || self.password.is_empty() || self.school_id.is_empty() {
            return;
        }
        write!(
            cred_file,
            "{}",
            toml::to_string(&Users::from(vec![self.clone()])).expect("couldn't serialize user")
        )
        .expect("couldn't save user");
    }

    /// load [`User`] with [`User::username`] or [`User::name()`] and save it to [`config_path()`]
    pub fn load(username: &str) -> Option<Self> {
        let mut matching_users = Vec::new();
        for user in Self::load_all() {
            if user
                .username
                .to_lowercase()
                .contains(&username.to_lowercase())
                || user
                    .name()
                    .to_lowercase()
                    .contains(&username.to_lowercase())
            {
                matching_users.push(user);
            }
        }
        let user = matching_users.first()?;

        user.save_to_conf();

        Some(user.clone())
    }
    /// save [`User`] as default to config.toml
    fn save_to_conf(&self) {
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
    /// load [`User`] configured in [`config_path()`]
    pub fn load_conf() -> Option<Self> {
        let conf_path = config_path()?;
        if !conf_path.exists() {
            return None;
        }
        let config_content = fs::read_to_string(conf_path).expect("couldn't read config file");
        let config = toml::from_str::<Config>(&config_content).expect("couldn't deser config");

        Self::load(&config.default_username)
    }

    /// get headers which are necessary for making certain requests
    fn headers(&self) -> AnyErr<HeaderMap> {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            format!("Bearer {}", self.token()?.access_token).parse()?,
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
    fn token(&self) -> AnyErr<Token> {
        // Define the key as bytes
        let key: &[u8] = &[98, 97, 83, 115, 120, 79, 119, 108, 85, 49, 106, 77];

        // Get nonce
        let nonce = blocking::get([endpoints::IDP, endpoints::NONCE].concat())?.text()?;

        // Define the message
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

        let client = Client::new();
        let res = client
            .post([endpoints::IDP, Token::ep()].concat())
            .headers(headers)
            .form(&data)
            .send()?;

        let text = res.text()?;
        let mut logf = File::create(log_path("token"))?;
        write!(logf, "{text}")?;

        let token = serde_json::from_str(&text)?;
        Ok(token)
    }

    /// Returns the current [`Lesson`]s of this [`User`].
    pub fn current_lessons(&self) -> Vec<Lesson> {
        if let Ok(lessons) = self.timetable(Local::now(), Local::now()) {
            lessons
        } else {
            vec![]
        }
    }

    /// get [`User`] info
    pub fn info(&self) -> AnyErr<Info> {
        let client = Client::new();
        let res = client
            .get(base(&self.school_id) + User::ep())
            .headers(self.headers()?)
            .send()?;
        let text = res.text()?;
        let mut logf = File::create(log_path("info"))?;
        write!(logf, "{text}")?;

        let info = serde_json::from_str(&text)?;
        Ok(info)
    }

    /// get all [`MsgOview`]s of a [`MsgKind`]
    pub fn msg_oviews_of_kind(&self, msg_kind: MsgKind) -> AnyErr<Vec<MsgOview>> {
        let client = Client::new();
        let res = client
            .get(endpoints::ADMIN.to_owned() + &endpoints::get_all_msgs(&msg_kind.val()))
            .headers(self.headers()?)
            .send()?;

        let text = res.text()?;
        let mut logf = File::create(log_path("messages"))?;
        write!(logf, "{text}")?;

        let msg = serde_json::from_str(&text)?;
        Ok(msg)
    }

    /// get all messages, of any kind
    pub fn all_msg_oviews(&self) -> AnyErr<Vec<MsgOview>> {
        let mut msgs = Vec::new();

        msgs = [msgs, self.msg_oviews_of_kind(MsgKind::Recv)?].concat();
        msgs = [msgs, self.msg_oviews_of_kind(MsgKind::Sent)?].concat();
        msgs = [msgs, self.msg_oviews_of_kind(MsgKind::Del)?].concat();

        Ok(msgs)
    }

    /// Get whole [`Msg`] from the `id` of a [`MsgOview`]
    pub fn full_msg(&self, msg_oview: &MsgOview) -> AnyErr<Msg> {
        let client = Client::new();
        let res = client
            .get(endpoints::ADMIN.to_owned() + &endpoints::get_msg(msg_oview.azonosito))
            .headers(self.headers()?)
            .send()?;

        let text = res.text()?;
        let mut logf = File::create(log_path("full_message"))?;
        write!(logf, "{text}")?;

        let msg = serde_json::from_str(&text)?;
        Ok(msg)
    }

    /// get all [`Eval`]s with `from` `to` or all
    pub fn evals(
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
        let client = Client::new();
        let res = client
            .get(base(&self.school_id) + Eval::ep())
            .query(&query)
            .headers(self.headers()?)
            .send()?;

        let text = res.text()?;
        let mut logf = File::create(log_path("evals"))?;
        write!(logf, "{text}")?;

        let evals = serde_json::from_str(&text)?;
        Ok(evals)
    }

    /// get all [`Lesson`]s `from` `to` which makes up a timetable
    pub fn timetable(&self, from: DateTime<Local>, to: DateTime<Local>) -> AnyErr<Vec<Lesson>> {
        let client = Client::new();
        let res = client
            .get(base(&self.school_id) + Lesson::ep())
            .query(&[("datumTol", from.to_string()), ("datumIg", to.to_string())])
            .headers(self.headers()?)
            .send()?;
        let text = res.text()?;

        let mut logf = File::create(log_path("timetable"))?;
        write!(logf, "{text}")?;

        let mut lessons = serde_json::from_str::<Vec<Lesson>>(&text)?;
        lessons.sort_by(|a, b| a.start().partial_cmp(&b.start()).expect("couldn't compare"));
        Ok(lessons)
    }

    /// get [`Announced`] tests `from` or all
    pub fn all_announced(&self, from: Option<DateTime<Utc>>) -> AnyErr<Vec<Announced>> {
        let query = if let Some(from) = from {
            vec![("datumTol", from.to_rfc3339())]
        } else {
            vec![]
        };
        let client = Client::new();
        let res = client
            .get(base(&self.school_id) + Announced::ep())
            .query(&query)
            .headers(self.headers()?)
            .send()?;

        let text = res.text()?;
        let mut logf = File::create(log_path("announced"))?;
        write!(logf, "{text}")?;

        let all_announced = serde_json::from_str(&text)?;
        Ok(all_announced)
    }

    /// Download all [`Attachment`]s of this [`Msg`].
    pub fn download_attachments(&self, msg: &Msg) -> AnyErr<()> {
        // let download_dir = dirs::download_dir().expect("couldn't find Downloads");
        for am in msg.attachments() {
            eprintln!("downloading {}", am.file_name);
            let mut f = File::create(am.file_name)?;

            let client = Client::new();
            client
                .get(endpoints::ADMIN.to_owned() + &endpoints::download_attachment(am.id))
                .headers(self.headers()?)
                .send()?
                .copy_to(&mut f)?;

            eprintln!("{}", endpoints::download_attachment(am.id));
        }
        Ok(())
    }

    /// get information about being [`Abs`]ent `from` `to` or all
    pub fn absences(
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
        let client = Client::new();
        let res = client
            .get(base(&self.school_id) + Abs::ep())
            .query(&query)
            .headers(self.headers()?)
            .send()?;

        let text = res.text()?;
        let mut logf = File::create(log_path("absences"))?;
        write!(logf, "{text}")?;

        let abss = serde_json::from_str(&text)?;
        Ok(abss)
    }

    /// get groups the [`User`] is a member of
    pub fn groups(&self) -> AnyErr<String> {
        let client = Client::new();
        let res = client
            .get(base(&self.school_id) + endpoints::CLASSES)
            .headers(self.headers()?)
            .send()?;

        let text = res.text()?;
        let mut logf = File::create(log_path("groups"))?;
        write!(logf, "{text}")?;

        // let all_announced = serde_json::from_str(&text)?;
        Ok(text)
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
mod tests {
    use super::*;

    #[test]
    fn deser_user() {
        let user = toml::from_str(
            r#"
            username = "Test Paul"
            password = "2000.01.01"
            school_id = "klik0000001"
            "#,
        );
        assert_eq!(
            Ok(User::new("Test Paul", "2000.01.01", "klik0000001")),
            user
        );
    }

    #[test]
    fn ser_user() {
        let user = User::new("Test Paul", "2000.01.01", "klik0000001");

        let user_toml = r#"username = "Test Paul"
password = "2000.01.01"
school_id = "klik0000001"
"#;

        assert_eq!(Ok(user_toml.to_owned()), toml::to_string(&user));
    }

    #[test]
    fn ser_users() {
        let users: Users = vec![
            User::new("Test Paul", "2000.01.01", "klik0000001"),
            User::new("Test Paulina", "2000.01.02", "klik0000002"),
        ]
        .into();

        let user_toml = r#"[[users]]
username = "Test Paul"
password = "2000.01.01"
school_id = "klik0000001"

[[users]]
username = "Test Paulina"
password = "2000.01.02"
school_id = "klik0000002"
"#;

        assert_eq!(Ok(user_toml.to_owned()), toml::to_string(&users));
    }

    #[test]
    fn deser_users() {
        let users: Users = vec![
            User::new("Test Paul", "2000.01.01", "klik0000001"),
            User::new("Test Paulina", "2000.01.02", "klik0000002"),
        ]
        .into();

        let user_toml = r#"[[users]]
username = "Test Paul"
password = "2000.01.01"
school_id = "klik0000001"

[[users]]
username = "Test Paulina"
password = "2000.01.02"
school_id = "klik0000002"
"#;

        assert_eq!(toml::to_string(&users), Ok(user_toml.to_owned()));
    }

    #[test]
    fn config_ser() {
        let config = Config {
            default_username: "Me Me Me!".to_owned(),
        };
        let config_toml = r#"default_username = "Me Me Me!"
"#;
        assert_eq!(Ok(config_toml.to_owned()), toml::to_string(&config));
    }
    #[test]
    fn config_deser() {
        let config_toml = r#"default_username = "Me Me Me!"
"#;
        let config = Config {
            default_username: "Me Me Me!".to_owned(),
        };
        assert_eq!(toml::from_str(config_toml), Ok(config));
    }
}
