use chrono::{Datelike, Local, Timelike};
use ekreta::LDateTime;

/// `DateTime` methods needed for `RsFilc`
pub trait MyDate {
    /// Practical date format.
    fn pretty(&self) -> String;
    /// Get hungarian month.
    fn hun_month<'a>(&self) -> &'a str;
    /// Get hungarian day of week.
    fn hun_day_of_week<'a>(&self) -> &'a str;
    fn make_kreta_valid(&self) -> String;
    fn to_day_with_hms(&self) -> LDateTime;
    fn day_diff(&self, other: &Self) -> Option<String>;
}

impl MyDate for LDateTime {
    fn pretty(&self) -> String {
        let this_year = self.year() == Local::now().year();

        if !this_year {
            format!("{}", self.format("%Y.%m.%d"))
        } else if let Some(day_diff) = self.day_diff(&Local::now()) {
            format!(
                "{} {}",
                day_diff,
                self.format(if self.hour() == 0 && self.minute() == 0 {
                    "%d."
                } else {
                    "%d. %H:%M"
                })
            )
        } else {
            format!(
                "{} {}",
                self.hun_month(),
                self.format(if self.hour() == 0 && self.minute() == 0 {
                    "%d."
                } else {
                    "%d. %H:%M"
                })
            )
        }
    }

    fn hun_month<'a>(&self) -> &'a str {
        match self.month() {
            1 => "jan.",
            2 => "feb.",
            3 => "már.",
            4 => "ápr.",
            5 => "máj.",
            6 => "jún.",
            7 => "júl.",
            8 => "aug.",
            9 => "szep.",
            10 => "okt.",
            11 => "nov.",
            12 => "dec.",
            _ => unreachable!("invalid month"),
        }
    }

    fn hun_day_of_week<'a>(&self) -> &'a str {
        match self.weekday().number_from_monday() {
            1 => "hétfő",
            2 => "kedd",
            3 => "szerda",
            4 => "csütörtök",
            5 => "péntek",
            6 => "szombat",
            7 => "vasárnap",
            _ => unreachable!("invalid day of week"),
        }
    }

    /// make [`LDateTime`] valid for `Datum(Ig|Tol)` for `Kréta`
    ///
    /// # warning
    ///
    /// only day start: 00:00:00 is valid for `Kréta` feck it
    fn make_kreta_valid(&self) -> String {
        self.date_naive().and_hms_opt(0, 0, 0).unwrap().to_string()
    }
    fn to_day_with_hms(&self) -> LDateTime {
        self.date_naive()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_local_timezone(Local)
            .unwrap()
    }

    fn day_diff(&self, other: &Self) -> Option<String> {
        let day_diff = self.num_days_from_ce() - other.num_days_from_ce();
        match day_diff {
            -2 => Some("tegnapelőtt".into()),
            -1 => Some("tegnap".into()),
            0 => Some("ma".into()),
            1 => Some("holnap".into()),
            2 => Some("holnapután".into()),
            _ => None,
        }
    }
}
