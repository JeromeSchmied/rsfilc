pub mod consts;
mod endpoints;
mod types;
mod user;

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
pub use http::header::{self, HeaderMap};
pub use user::User;

/// optional interval
pub type OptIrval = (Option<LDateTime>, Option<LDateTime>);
/// Local DateTime
pub type LDateTime = chrono::DateTime<chrono::Local>;
// quite universal error type, although do consider migrating to `anyhow`
pub type Res<T> = Result<T, Box<dyn std::error::Error>>;
