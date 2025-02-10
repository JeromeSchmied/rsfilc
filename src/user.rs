use crate::{config::Config, time::MyDate, timetable::next_lesson, *};
use base64::{engine::general_purpose::STANDARD, Engine};
use chrono::{DateTime, Datelike, Days, Local, NaiveDate, Utc};
use ekreta::{
    consts, header, Absence, AnnouncedTest as Ancd, Evaluation as Eval, HeaderMap, Lesson, MsgItem,
    OptIrval, Token,
};
use log::{info, warn};
use serde::{Deserialize, Serialize};
use std::{
    fmt::Debug,
    io::{self, Write},
};

pub fn handle(
    username: Option<String>,
    create: bool,
    conf: &mut Config,
    del: bool,
    switch: bool,
) -> Res<()> {
    if let Some(name) = username {
        if create {
            Usr::create(name, conf);
            println!("created");
        } else if del {
            conf.delete(&name);
            println!("deleted");
        } else if switch {
            conf.switch_user_to(name);
            println!("switched");
        }
        conf.save()?;
    } else {
        println!("nem mondtad meg mit/kivel kell csinálni, felsorolom a");
        println!("\nFelhasználókat:\n");
        for current_user in &conf.users {
            // definitely overkill, but does the job ;)
            let user_info = current_user.0.fetch_info(&current_user.headers()?)?;
            let as_str = information::disp(&user_info, &current_user.0.username);
            println!("\n\n{as_str}");
            fill(&as_str, '-', None);
        }
    }
    Ok(())
}

/// Kréta, app user
#[derive(Clone, PartialOrd, Ord, Eq, PartialEq, Deserialize, Serialize, Debug)]
pub struct Usr(pub ekreta::User);
// basic stuff
impl Usr {
    /// get name of [`User`]
    ///
    /// # Errors
    ///
    /// net
    pub fn name(&self) -> Res<String> {
        Ok(self.0.fetch_info(&self.headers()?)?.nev)
    }

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
        user.save(conf);
        Some(user)
    }

    /// save [`User`] credentials
    /// also set as default
    fn save(&self, conf: &mut Config) {
        conf.users.insert(self.clone());
        conf.switch_user_to(self.0.username.clone());
    }

    /// load default [`User`]
    pub fn load(conf: &Config) -> Option<Self> {
        conf.users
            .iter()
            .find(|u| u.0.username == conf.default_username)
            .cloned()
    }

    /// print all lessons of a day
    pub fn print_day(&self, mut lessons: Vec<Lesson>) {
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
                .get_all_announced((Some(first_lesson.kezdet_idopont.date_naive()), None))
                .expect("couldn't fetch announced tests");
            let all_lessons_till_day = self
                .get_timetable(first_lesson.kezdet_idopont.date_naive(), true)
                .unwrap_or_default();
            // info!("all announced: {todays_tests:?}");

            // number of lessons at the same time
            lessons.retain(|l| !l.kamu_smafu());

            for (n, lesson) in lessons.iter().enumerate() {
                // calculate `n`. this lesson is
                let nth = lesson.oraszam.unwrap_or(u8::MAX);
                if n as u8 + 2 == nth
                    && lessons
                        .get(n - 1)
                        .is_none_or(|prev| prev.oraszam.unwrap_or(u8::MAX) == n as u8)
                {
                    let no_lesson_buf = format!(
                        "\n\n{}. Lyukas (avagy Lukas) óra\n| Erre az időpontra nincsen tanóra rögzítve.",
                        n + 1
                    );
                    println!("{no_lesson_buf}");
                    fill(&no_lesson_buf, '^', Some("Juhé"));
                }
                // so fill_under() works fine
                let mut printer = format!("\n\n{nth}. {}", timetable::disp(lesson));

                if let Some(test) = todays_tests
                    .iter()
                    .find(|ancd| ancd.orarendi_ora_oraszama.is_some_and(|x| x == nth))
                {
                    printer += &format!(
                        "\n| {}{}",
                        test.modja.leiras,
                        if let Some(topic) = test.temaja.as_ref() {
                            format!(": {topic}")
                        } else {
                            String::new()
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
impl Usr {
    /// convert type name of `T` to a kind name, used for cache
    fn type_to_kind_name<T>() -> Res<String> {
        let type_name = std::any::type_name::<T>();
        let kind = type_name.split("::").last().ok_or("invalid type_name")?;
        let kind = kind.trim_matches(['<', '>']).to_ascii_lowercase();
        Ok(kind)
    }
    /// helper fn, stores `content` of `kind` to `self.0.username` cache-dir
    fn store_cache<S: Serialize>(&self, content: &S) -> Res<()> {
        let kind = Self::type_to_kind_name::<S>()?;

        let content = serde_json::to_string(content)?;
        cache::store(&self.0.username, &kind, &content)
    }
    /// helper fn, loads cache of `kind` from `self.0.username` cache-dir
    fn load_cache<D: for<'a> Deserialize<'a>>(&self) -> Option<(DateTime<Utc>, D)> {
        let kind = Self::type_to_kind_name::<D>().ok()?;

        let (cache_t, content) = cache::load(&self.0.username, &kind)?;
        let deserd = serde_json::from_str(&content).ok()?;
        Some((cache_t, deserd))
    }
    fn fetch_vec<E>(&self, query: E::Args) -> Res<Vec<E>>
    where
        E: ekreta::Endpoint + for<'a> serde::Deserialize<'a>,
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
            if Utc::now().signed_duration_since(cache_t)
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

    /// get all [`Eval`]s with `from` `to` or all
    ///
    /// # Panics
    ///
    /// sorting
    ///
    /// # Errors
    ///
    /// net
    pub fn get_evals(&self, interval: OptIrval) -> Res<Vec<Eval>> {
        let (cache_t, cached_evals) = self.load_cache::<Vec<Eval>>().unzip();
        let mut evals = cached_evals.unwrap_or_default();

        let interval = fix_irval(cache_t, interval);

        let mut fetch_err = false;
        let fetched_evals = self.fetch_vec(interval).inspect_err(|e| {
            fetch_err = true;
            warn!("couldn't fetch from E-Kréta server: {e:?}");
        });

        info!("received evals");

        evals.extend(fetched_evals.unwrap_or_default());
        evals.sort_unstable_by_key(|e| e.keszites_datuma);
        evals.dedup_by_key(|e| e.keszites_datuma);
        if interval.0.is_none() && !fetch_err {
            self.store_cache(&evals)?;
        }
        Ok(evals)
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
        let fetched_lessons_week = self
            .fetch_vec((week_start, week_end))
            .inspect_err(|e| {
                fetch_err = true;
                warn!("couldn't deserialize data: {e:?}");
            })
            .unwrap_or_default();

        lessons.extend(fetched_lessons_week);
        lessons.sort_unstable_by_key(|l| l.kezdet_idopont);
        lessons.dedup_by_key(|l| l.kezdet_idopont);
        if !fetch_err {
            self.store_cache(&lessons)?;
        }
        if !everything_till_day {
            lessons.retain(|lsn| lsn.kezdet_idopont.date_naive() == day);
        }
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
    pub fn get_all_announced(&self, interval: OptIrval) -> Res<Vec<Ancd>> {
        let (cache_t, cached_ancd) = self.load_cache::<Vec<Ancd>>().unzip();
        let mut tests = cached_ancd.unwrap_or_default();

        let interval = fix_irval(cache_t, interval);

        let mut fetch_err = false;
        let fetched_tests = self.fetch_vec(interval).inspect_err(|e| {
            fetch_err = true;
            warn!("couldn't reach E-Kréta server: {e:?}");
        });

        tests.extend(fetched_tests.unwrap_or_default());
        tests.sort_unstable_by_key(|a| a.datum);
        tests.dedup_by_key(|a| a.datum);
        if let Some(from) = interval.0 {
            info!("filtering, from!");
            tests.retain(|ancd| ancd.datum.num_days_from_ce() >= from.num_days_from_ce());
        }
        if let Some(to) = interval.1 {
            info!("filtering, to!");
            tests.retain(|ancd| ancd.datum.num_days_from_ce() <= to.num_days_from_ce());
        }
        if interval.0.is_none() && !fetch_err {
            self.store_cache(&tests)?;
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
    pub fn get_absences(&self, interval: OptIrval) -> Res<Vec<Absence>> {
        let (cache_t, cached_abs) = self.load_cache::<Vec<Absence>>().unzip();
        let mut absences = cached_abs.unwrap_or_default();

        let interval = fix_irval(cache_t, interval);

        let mut fetch_err = false;
        let fetched_absences = self.fetch_vec(interval).inspect_err(|e| {
            fetch_err = true;
            warn!("couldn't fetch from E-Kréta server: {e:?}");
        });

        info!("received absences");
        absences.extend(fetched_absences.unwrap_or_default());
        absences.sort_unstable_by_key(|a| a.ora.kezdo_datum);

        if interval.0.is_none() && !fetch_err {
            self.store_cache(&absences)?;
        }

        Ok(absences)
    }
}

/// use `cache_t` as `interval.0` (from) if some
fn fix_irval(cache_t: Option<DateTime<Utc>>, mut interval: OptIrval) -> OptIrval {
    if let Some(ct) = cache_t {
        info!("from cached");
        interval.0 = Some(ct.date_naive());
    }
    interval
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
                info!("not downloading, already done");
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
    pub fn msgs(&self, interval: OptIrval) -> Res<Vec<MsgItem>> {
        let (cache_t, cached_msg) = self.load_cache::<Vec<MsgItem>>().unzip();
        let mut msgs = cached_msg.unwrap_or_default();

        let (from, _) = fix_irval(cache_t, interval);

        let mut fetched_msgs = Vec::new();
        let mut handles = Vec::new();

        let mut fetch_err = false;
        for msg_oview in self
            .0
            .fetch_msg_oviews(&self.headers()?)
            .unwrap_or_default()
        {
            // if isn't between `from`-`to`
            if from.is_some_and(|fm| msg_oview.uzenet_kuldes_datum < fm.into())
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
                        fetch_err = true;
                        warn!("couldn't fetch from E-Kréta server: {e:?}");
                    })
                    .unwrap()
            });
            handles.push(h);
        }

        let mut am_handles = Vec::new();

        for h in handles {
            fetched_msgs.push(h.join().unwrap());
        }

        msgs.extend(fetched_msgs);
        msgs.sort_unstable_by_key(|m| m.uzenet.kuldes_datum);
        msgs.dedup();
        for msg in msgs.clone() {
            let s = self.clone();
            let xl = std::thread::spawn(move || {
                s.download_attachments(&msg)
                    .inspect_err(|e| warn!("couldn't fetch from E-Kréta server: {e:?}"))
                    .unwrap();
            });
            am_handles.push(xl);
        }
        for h in am_handles {
            let j = h.join();
            j.map_err(|e| *e.downcast::<String>().unwrap())?;
        }
        if interval.0.is_none() && !fetch_err {
            self.store_cache(&msgs)?;
        }
        Ok(msgs)
    }

    /// get notes: additional messages the [`User`] received.
    ///
    /// # Errors
    ///
    /// - net
    pub fn get_note_msgs(&self, interval: OptIrval) -> Res<Vec<ekreta::NoteMsg>> {
        let (cache_t, cached_notemsgs) = self.load_cache::<Vec<ekreta::NoteMsg>>().unzip();
        let mut note_msgs = cached_notemsgs.unwrap_or_default();

        let interval = fix_irval(cache_t, interval);

        let mut fetch_err = false;
        let fetched_note_msgs = self.fetch_vec(interval).inspect_err(|e| {
            fetch_err = true;
            warn!("couldn't reach E-Kréta server: {e:?}");
        });

        note_msgs.extend(fetched_note_msgs.unwrap_or_default());
        if interval.0.is_none() && !fetch_err {
            self.store_cache(&note_msgs)?;
        }

        Ok(note_msgs)
    }
}
