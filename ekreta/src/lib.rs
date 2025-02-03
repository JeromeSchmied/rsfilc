pub mod consts;
mod endpoints;
mod types;
mod user;
// mod error;
pub type Res<T> = Result<T, Box<dyn std::error::Error>>;

pub use endpoints::absences::Absence;
pub use endpoints::announced_tests::AnnouncedTest;
pub use endpoints::evaluations::Evaluation;
pub use endpoints::groups::Class;
pub use endpoints::lessons::Lesson;
pub use endpoints::messages::{
    Attachment, Message, MessageItem, MessageKind, MessageOverview, NoteMessage,
};
pub use endpoints::schools::School;
pub use endpoints::token::Token;
pub use endpoints::user_info::UserInfo;
pub use endpoints::Endpoint;
use std::borrow::Cow;
// pub use error::Error;

/// optional interval
pub type OptIrval = (Option<LDateTime>, Option<LDateTime>);
/// Local DateTime
pub type LDateTime = chrono::DateTime<chrono::Local>;
pub type Str = Cow<'static, str>;
pub type Bytes = Cow<'static, [u8]>;
