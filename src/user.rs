use crate::{config::Config, time::MyDate, timetable::next_lesson, *};
use base64::{engine::general_purpose::STANDARD, Engine};
use chrono::{Datelike, Days, Local, NaiveDate};
use ekreta::{
    consts, header, Absence, AnnouncedTest as Ancd, Evaluation as Eval, HeaderMap, LDateTime,
    Lesson, MsgItem, OptIrval, Token,
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
            cache::delete_dir()?;
            let user_info = current_user.0.fetch_info(&current_user.headers()?)?;
            let as_str = information::disp(&user_info, &current_user.0.username);
            println!("\n\n{as_str}");
            fill(&as_str, '-', None);
        }
        cache::delete_dir()?;
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
                .get_all_announced((
                    Some(first_lesson.kezdet_idopont),
                    Some(lessons.last().unwrap().veg_idopont),
                ))
                .expect("couldn't fetch announced tests");
            let all_lessons_till_day = self
                .get_timetable(first_lesson.kezdet_idopont.date_naive(), true)
                .unwrap_or_default();
            // info!("all announced: {todays_tests:?}");

            // number of lessons at the same time
            lessons.retain(|l| !l.kamu_smafu());

            for (n, lesson) in lessons.iter().enumerate() {
                // calculate `n`. this lesson is
                let nth = lesson.oraszam;
                if n as u8 + 2 == nth
                    && lessons
                        .get(n - 1)
                        .is_none_or(|prev| prev.oraszam == n as u8)
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
    /// get headers which are necessary for making certain requests
    pub fn headers(&self) -> Res<HeaderMap> {
        Ok(HeaderMap::from_iter([
            (
                header::AUTHORIZATION,
                format!("Bearer {}", self.get_token()?.access_token).parse()?,
            ),
            (header::USER_AGENT, consts::USER_AGENT.parse()?),
        ]))
    }

    fn get_token(&self) -> Res<Token> {
        if let Some((cache_t, cache_content)) = cache::load("token") {
            let cached_token: Token = serde_json::from_str(&cache_content)?;
            if Local::now().signed_duration_since(cache_t)
                < chrono::Duration::seconds(cached_token.expires_in.into())
            {
                return Ok(cached_token);
            }
            // refresh token
            let response =
                ekreta::User::refresh_token_resp(&self.0.schoolid, &cached_token.refresh_token)?;
            let txt = response.text()?;

            let token = serde_json::from_str(&txt)?;
            cache::store("token", &txt)?;
            return Ok(token);
        }
        let decoded_password = self.decode_password();
        let tmp_user = ekreta::User {
            password: decoded_password,
            ..self.clone().0
        };
        let resp = tmp_user.get_token_resp()?;
        let text = resp.text()?;

        let token = serde_json::from_str(&text)?;
        cache::store("token", &text)?;
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
    pub fn get_evals(&self, mut interval: OptIrval) -> Res<Vec<Eval>> {
        let (cache_t, cache_content) = cache::load("evals").unzip();
        let mut evals = if let Some(cached) = &cache_content {
            serde_json::from_str::<Vec<Eval>>(cached)?
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
            .0
            .fetch_evals(interval, &self.headers()?)
            .inspect_err(|e| {
                fetch_err = true;
                warn!("couldn't fetch from E-Kréta server: {e:?}");
            });

        info!("received evals");

        evals.extend(fetched_evals.unwrap_or_default());
        evals.sort_by(|a, b| b.keszites_datuma.partial_cmp(&a.keszites_datuma).unwrap());
        evals.dedup();
        if interval.0.is_none() && !fetch_err {
            cache::store("evals", &serde_json::to_string(&evals)?)?;
        }
        Ok(evals)
    }

    pub fn get_timetable(&self, day: NaiveDate, everything_till_day: bool) -> Res<Vec<Lesson>> {
        let (_, cache_content) = cache::load("timetable").unzip();
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
        lessons.sort_unstable_by_key(|l| l.kezdet_idopont);
        lessons.dedup_by_key(|l| l.kezdet_idopont);
        if !fetch_err {
            cache::store("timetable", &serde_json::to_string(&lessons)?)?;
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
    fn fetch_timetable(&self, from: LDateTime, to: LDateTime) -> Res<Vec<Lesson>> {
        let mut lessons = self.0.fetch_timetable((from, to), &self.headers()?)?;
        info!("received lessons");
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
    pub fn get_all_announced(&self, mut interval: OptIrval) -> Res<Vec<Ancd>> {
        let (cache_t, cache_content) = cache::load("announced").unzip();
        let mut tests = if let Some(cached) = &cache_content {
            serde_json::from_str::<Vec<Ancd>>(cached)?
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
            .0
            .fetch_announced_tests(interval, &self.headers()?)
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
            cache::store("announced", &serde_json::to_string(&tests)?)?;
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
    pub fn get_absences(&self, mut interval: OptIrval) -> Res<Vec<Absence>> {
        let (cache_t, cache_content) = cache::load("absences").unzip();
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
            .0
            .fetch_absences(interval, &self.headers()?)
            .inspect_err(|e| {
                fetch_err = true;
                warn!("couldn't fetch from E-Kréta server: {e:?}");
            });

        info!("received absences");
        absences.extend(fetched_absences.unwrap_or_default());
        absences.sort_by(|a, b| b.ora.kezdo_datum.partial_cmp(&a.ora.kezdo_datum).unwrap());

        if interval.0.is_none() && !fetch_err {
            cache::store("absences", &serde_json::to_string(&absences)?)?;
        }

        Ok(absences)
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
                info!("not downloading, already done");
                continue;
            }
            self.0
                .download_attachment_to(am.azonosito, download_to, &self.headers()?)?;

            info!("received file {}", &am.fajl_nev);
        }
        Ok(())
    }

    // pub fn fetch_messages(&self) -> Res<Vec<MsgItem>> {
    //     let msgs = self.fetch_vec((), "")?;
    //     Ok(msgs)
    // }
    /// Fetch max `n` [`Msg`]s between `from` and `to`.
    /// Also download all `[Attachment]`s each [`Msg`] has.
    ///
    /// # Errors
    ///
    /// - net
    pub fn msgs(&self, interval: OptIrval) -> Res<Vec<MsgItem>> {
        let (cache_t, cache_content) = cache::load("messages").unzip();
        let mut msgs = if let Some(cached) = &cache_content {
            serde_json::from_str::<Vec<MsgItem>>(cached)?
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
        for msg_oview in self
            .0
            .fetch_msg_oviews(&self.headers()?)
            .unwrap_or_default()
        {
            // if isn't between `from`-`to`
            if from.is_some_and(|fm| msg_oview.uzenet_kuldes_datum < fm.date_naive().into())
                || interval
                    .1
                    .is_some_and(|to| msg_oview.uzenet_kuldes_datum > to.date_naive().into())
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
            cache::store("messages", &serde_json::to_string(&msgs)?)?;
        }
        Ok(msgs)
    }

    /// get notes: additional messages the [`User`] received.
    ///
    /// # Errors
    ///
    /// - net
    pub fn get_note_msgs(&self, mut interval: OptIrval) -> Res<Vec<ekreta::NoteMsg>> {
        let (cache_t, cache_content) = cache::load("note_messages").unzip();
        let mut note_msgs = if let Some(cached) = &cache_content {
            serde_json::from_str::<Vec<ekreta::NoteMsg>>(cached)?
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
        let fetched_note_msgs = self
            .0
            .fetch_note_msgs(interval, &self.headers()?)
            .inspect_err(|e| {
                fetch_err = true;
                warn!("couldn't reach E-Kréta server: {e:?}");
            });

        note_msgs.extend(fetched_note_msgs.unwrap_or_default());
        if interval.0.is_none() && !fetch_err {
            cache::store("note_messages", &serde_json::to_string(&note_msgs)?)?;
        }

        Ok(note_msgs)
    }
}
