use crate::{config::Config, *};
use base64::{engine::general_purpose::STANDARD, Engine};
use chrono::{Datelike, Days, Local, NaiveDate};
use ekreta::{
    consts, header, Absence, AnnouncedTest as Ancd, Evaluation as Eval, HeaderMap, Lesson, MsgItem,
    OptIrval, Token,
};
use serde::{Deserialize, Serialize};
use std::io::{self, Write};

pub fn handle(
    userid: Option<String>,
    create: bool,
    conf: &mut Config,
    del: bool,
    switch: bool,
    cache_dir: bool,
    json: bool,
) -> Res<()> {
    if let Some(name) = userid {
        if create {
            let res = Usr::create(name.clone(), conf)
                .ok_or("couldn't create user, check your credentials and network connection");
            // delete cache dir if couldn't log in
            if res.is_err() {
                crate::cache::delete_dir(&name)?;
            }
            res?;
            println!("created");
        } else {
            let name = conf
                .get_userid(name)
                .ok_or("the given userid/name isn't saved")?;
            if del {
                conf.delete(name);
                println!("deleted");
            } else if switch {
                conf.switch_user_to(&name);
                println!("switched");
            } else if cache_dir {
                let cache_dir = paths::cache_dir(&name).ok_or("no cache dir found for user")?;
                println!("{}", cache_dir.display());
            }
        }
        conf.save()?;
    } else {
        if cache_dir {
            let cache_dir =
                paths::cache_dir(&conf.default_username).ok_or("no cachedir found of for user")?;
            println!("{}", cache_dir.display());
            return Ok(());
        }
        if json {
            let json = serde_json::to_string(&(&conf.default_username, &conf.users))?;
            println!("{json}");
        } else {
            println!("Felhasználók:");
            information::display(conf)?;
        }
    }
    Ok(())
}

/// Kréta, app user
#[derive(Clone, PartialOrd, Ord, Eq, PartialEq, Deserialize, Serialize, Debug)]
pub struct Usr(pub ekreta::User);
// basic stuff
impl Usr {
    /// create new instance of [`User`]
    pub fn new(username: String, password: String, schoolid: String) -> Self {
        let password = STANDARD.encode(password);
        Self(ekreta::User {
            username,
            password,
            schoolid,
        })
    }
    /// Returns the decoded password of this [`User`].
    ///
    /// # Panics
    ///
    /// Panics if decode fails.
    fn decode_password(&self) -> String {
        let decoded_password = STANDARD.decode(&self.0.password).unwrap();
        String::from_utf8(decoded_password).unwrap()
    }
    /// creates dummy [`User`], that won't be saved and shouldn't be used
    pub fn dummy() -> Self {
        info!("created dummy user");
        Self::new(String::new(), String::new(), String::new())
    }

    /// create a [`User`] from cli and write it to `conf`!
    ///
    /// # Errors
    ///
    /// `std::io::std(in/out)`
    pub fn create(username: String, conf: &mut Config) -> Option<Self> {
        info!("creating user from cli");
        info!("received username from cli");

        let Ok(password) = rpassword::prompt_password("password: ") else {
            println!("password is required");
            return None;
        };
        info!("received password {} from cli", "*".repeat(password.len()));

        print!("schoolid: ");
        io::stdout().flush().ok()?;
        let mut schoolid = String::new();
        io::stdin().read_line(&mut schoolid).ok()?;
        let schoolid = schoolid.trim().to_string();
        if schoolid.is_empty() {
            println!("schoolid is required");
            return None;
        }
        info!("received schoolid {schoolid} from cli");

        let user = Self::new(username, password, schoolid);
        user.get_userinfo().ok()?;
        user.save(conf);
        Some(user)
    }

    /// save [`User`] credentials
    /// also set as default
    fn save(&self, conf: &mut Config) {
        conf.users.insert(self.clone());
        conf.switch_user_to(&self.0.username);
    }

    /// load default [`User`]
    pub fn load(conf: &Config) -> Option<Self> {
        conf.users
            .iter()
            .find(|u| u.0.username == conf.default_username)
            .cloned()
    }
}

// interacting with API
impl Usr {
    /// helper fn, stores `content` of `kind` to `self.0.username` cache-dir
    fn store_cache<S: Serialize>(&self, content: &S) -> Res<()> {
        let kind = utils::type_to_kind_name::<S>()?;

        let content = serde_json::to_string(content)?;
        cache::store(&self.0.username, &kind, &content)
    }
    /// helper fn, loads cache of `kind` from `self.0.username` cache-dir
    fn load_cache<D: for<'a> Deserialize<'a>>(&self) -> Option<(ekreta::LDateTime, D)> {
        let kind = utils::type_to_kind_name::<D>().ok()?;
        if std::env::var("NO_CACHE").is_ok_and(|nc| nc == "1") && kind != "token" {
            log::info!("manually triggered 'no cache' error");
            return None;
        }

        let (cache_t, content) = cache::load(&self.0.username, &kind)?;
        let deserd = serde_json::from_str(&content)
            .inspect_err(|e| {
                error!("{e:?} - couldn't deserialize {kind}: {content}");
                eprintln!("error: {e:?}, check logs with `cat $(rsfilc --cache-dir)/rsfilc.log`");
            })
            .ok()?;
        Some((cache_t, deserd))
    }
    fn fetch_vec<E>(&self, query: E::Args) -> Res<Vec<E>>
    where
        E: ekreta::Endpoint + for<'a> Deserialize<'a>,
    {
        self.0.fetch_vec(query, &self.headers()?)
    }
    /// get headers which are necessary for making certain requests
    fn headers(&self) -> Res<HeaderMap> {
        Ok(HeaderMap::from_iter([
            (
                header::AUTHORIZATION,
                format!("Bearer {}", self.get_token()?.access_token).parse()?,
            ),
            (header::USER_AGENT, consts::USER_AGENT.parse()?),
        ]))
    }

    fn get_token(&self) -> Res<Token> {
        if let Some((cache_t, cached_token)) = self.load_cache::<Token>() {
            if Local::now().signed_duration_since(cache_t)
                < chrono::Duration::seconds(cached_token.expires_in.into())
            {
                return Ok(cached_token);
            }
            // refresh token
            let token = self.0.refresh_token(&cached_token.refresh_token)?;
            self.store_cache(&token)?;
            return Ok(token);
        }
        let authed_user = ekreta::User {
            password: self.decode_password(),
            ..self.clone().0
        };
        let token = authed_user.fetch_token()?;
        self.store_cache(&token)?;
        info!("received token");
        Ok(token)
    }
    pub fn get_userinfo(&self) -> Res<ekreta::UserInfo> {
        if let Some((_, cached_info)) = self.load_cache::<ekreta::UserInfo>() {
            Ok(cached_info)
        } else {
            let fetched_info = self.0.fetch_info(&self.headers()?)?;
            self.store_cache(&fetched_info)?;
            Ok(fetched_info)
        }
    }

    gen_get_for! { get_evals, Eval, false,
        (|evals: &mut Vec<Eval>| {
            evals.sort_unstable_by_key(|e| e.keszites_datuma);
            evals.dedup_by_key(|e| e.uid.clone());
        })
    }

    pub fn get_timetable(&self, day: NaiveDate, everything_till_day: bool) -> Res<Vec<Lesson>> {
        let (_, cache_tt) = self.load_cache::<Vec<Lesson>>().unzip();
        let mut lessons = cache_tt.unwrap_or_default();
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

        let mut fetch_err = false;
        let mut fetched_lessons_week: Vec<Lesson> = self
            .fetch_vec((week_start, week_end))
            .inspect_err(|e| {
                fetch_err = true;
                warn!("couldn't fetch or deserialize data: {e:?}");
            })
            .unwrap_or_default();
        lessons.retain(|l| {
            !fetched_lessons_week
                .iter()
                .any(|fl| l.kezdet_idopont == fl.kezdet_idopont && l.nev == fl.nev)
        });

        lessons.append(&mut fetched_lessons_week);
        lessons.sort_unstable_by_key(|l| l.kezdet_idopont);
        if !fetch_err {
            self.store_cache(&lessons)?;
        }
        if !everything_till_day {
            lessons.retain(|lsn| lsn.kezdet_idopont.date_naive() == day);
        }
        Ok(lessons)
    }

    gen_get_for! { get_tests, Ancd, false,
        (|tests: &mut Vec<Ancd>| {
            tests.sort_unstable_by_key(|a| (a.datum, a.uid.clone()));
            tests.dedup_by_key(|a| a.uid.clone());
        })
    }

    gen_get_for! { get_absences, Absence, true,
        (|absences: &mut Vec<Absence>| {
                absences.sort_unstable_by_key(|a| (a.ora.kezdo_datum, !a.igazolt()));
                absences.dedup_by_key(|a| a.ora.clone());
        })
    }
}

/// [`Msg`]s and [`Attachment`]s
impl Usr {
    /// Download all [`Attachment`]s of this [`Msg`] to [`download_dir()`].
    ///
    /// # Errors
    /// - net
    pub fn download_attachments(&self, msg: &MsgItem) -> Res<()> {
        for am in &msg.uzenet.csatolmanyok {
            let download_to = messages::download_attachment_to(am);
            info!("downloading file://{}", download_to.display());
            // don't download if already exists
            if download_to.exists() {
                debug!("not downloading, already done");
                continue;
            }
            self.0
                .download_attachment_to(am.azonosito, download_to, &self.headers()?)?;

            info!("received file {}", &am.fajl_nev);
        }
        Ok(())
    }

    /// Fetch max `n` [`Msg`]s between `from` and `to`.
    /// Also download all `[Attachment]`s each [`Msg`] has.
    ///
    /// # Errors
    ///
    /// - net
    pub fn msgs(&self, mut interval: OptIrval) -> Res<Vec<MsgItem>> {
        let (cache_t, cached_msg) = self.load_cache::<Vec<MsgItem>>().unzip();
        if cache_t.is_some() {
            interval = utils::fix_from(cache_t, interval)
        }
        let mut msgs;

        match self.fetch_msgs(interval) {
            Ok(fetched_msgs) => {
                msgs = cached_msg.unwrap_or_default();
                msgs.extend(fetched_msgs);
                msgs.sort_unstable_by_key(|m| m.uzenet.kuldes_datum);
                msgs.dedup_by_key(|m| m.azonosito);

                if interval.0.is_none() && !msgs.is_empty() {
                    self.store_cache(&msgs)?;
                }
            }
            Err(e) => {
                error!("couldn't reach E-Kréta server: {e:?}, only loading cached messages");
                eprintln!("couldn't reach E-Kréta server: {e:?}, only loading cached messages");
                msgs = cached_msg.ok_or("nothing cached")?;
            }
        }

        self.download_all_attachments(&msgs)?;

        Ok(msgs)
    }

    fn download_all_attachments(&self, msgs: &[MsgItem]) -> Res<()> {
        let mut am_handles = Vec::new();
        for msg in msgs.to_owned() {
            let usr = self.clone();
            let xl = std::thread::spawn(move || {
                usr.download_attachments(&msg)
                    .inspect_err(|e| error!("couldn't fetch from E-Kréta server: {e:?}"))
                    .unwrap();
            });
            am_handles.push(xl);
        }
        for h in am_handles {
            let j = h.join();
            j.map_err(|e| *e.downcast::<String>().unwrap())?;
        }
        Ok(())
    }

    fn fetch_msgs(&self, interval: OptIrval) -> Res<Vec<MsgItem>> {
        let mut fetched_msgs = Vec::new();
        let mut handles = Vec::new();
        for msg_oview in self.0.fetch_msg_oviews(&self.headers()?)? {
            // if isn't between `from`-`to`
            if interval
                .0
                .is_some_and(|fm| msg_oview.uzenet_kuldes_datum < fm.into())
                || interval
                    .1
                    .is_some_and(|to| msg_oview.uzenet_kuldes_datum > to.into())
            {
                continue;
            }
            let s = self.clone();
            let h = std::thread::spawn(move || {
                s.0.fetch_full_msg(Some(&msg_oview), &s.headers().unwrap())
                    .inspect_err(|e| {
                        warn!("couldn't fetch from E-Kréta server: {e:?}");
                    })
                    .unwrap()
            });
            handles.push(h);
        }
        for h in handles {
            fetched_msgs.push(h.join().map_err(|e| *e.downcast::<String>().unwrap())?);
        }
        Ok(fetched_msgs)
    }

    gen_get_for! { get_note_msgs, ekreta::NoteMsg, false,
        (|nmsgs: &mut Vec<ekreta::NoteMsg>| {
            nmsgs.sort_unstable_by_key(|nmsg| nmsg.datum);
            nmsgs.dedup();
        })
    }

    /// load data from cache, fetch remaining(or full, depending on `fix_irval`) interval, merge these two sources
    /// # NOTE
    /// - if any of the two fails, it will be logged, but ignored and the other source will be used instead
    /// - don't forget to deduplicate the returned Vec **properly**
    /// # WARN
    /// `irval.1` so the end of the interval, 'to' will be ignored in terms of cached data
    fn load_n_fetch<Ep>(&self, irval: &mut OptIrval, fix_irval: bool) -> Res<Vec<Ep>>
    where
        Ep: ekreta::Endpoint<Args = OptIrval> + for<'a> Deserialize<'a> + Clone,
    {
        let (cache_t, cached) = self.load_cache::<Vec<Ep>>().unzip();

        if fix_irval && cached.is_some() {
            *irval = utils::fix_from(cache_t, *irval);
        }

        let fetched = self.fetch_vec::<Ep>(*irval);

        match fetched {
            Ok(fetched_items) => {
                let mut cached = cached.unwrap_or_default();
                cached.retain(|item| {
                    item.when()
                        .is_none_or(|dt| irval.0.is_none_or(|from| dt.date_naive() <= from))
                });
                Ok([cached, fetched_items].concat())
            }
            Err(e) => {
                error!("couldn't reach E-Kréta server: {e:?}");
                eprintln!("couldn't reach E-Kréta server: {e:?}");
                let kind_name = utils::type_to_kind_name::<Ep>().unwrap_or_default();
                warn!("request error, only loading cached {kind_name}");
                eprintln!("request error, only loading cached {kind_name}");
                cached.ok_or("nothing cached".into())
            }
        }
    }
}
