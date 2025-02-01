mod endpoints;
mod types;
// mod error;

pub use anyhow::Result;
use chrono::{DateTime, Local};
use std::borrow::Cow;

pub use endpoints::absences::Absence;
pub use endpoints::announced_tests::AnnouncedTest;
pub use endpoints::evaluations::Evaluation;
pub use endpoints::Endpoint;
// pub use error::Error;

pub type Interval = (Option<DateTime<Local>>, Option<DateTime<Local>>);
pub type Str = Cow<'static, str>;
pub type Bytes = Cow<'static, [u8]>;
