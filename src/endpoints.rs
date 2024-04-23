//! `Kréta` API

/// base url of school with `school_id`
/// "https://{school_id}.e-kreta.hu"
pub fn base(school_id: &str) -> String {
    format!("https://{school_id}.e-kreta.hu")
}

/// kreta idp base Url
pub const IDP: &str = "https://idp.e-kreta.hu";
/// kreta admin base Url
pub const ADMIN: &str = "https://eugyintezes.e-kreta.hu";
/// kreta files base Url
pub const FILES: &str = "https://files.e-kreta.hu";
/// just a random `USER_AGENT`
pub const USER_AGENT: &str = "hu.ekreta.student/9.1.1/Linux/1";
/// client id, just like as if it was official
pub const CLIENT_ID: &str = "kreta-ellenorzo-mobile-android";

/// nonce
pub const NONCE: &str = "/nonce";
/// what are these?
pub const NOTES: &str = "/ellenorzo/V3/Sajat/Feljegyzesek";
/// what are these?
pub const EVENTS: &str = "/ellenorzo/V3/Sajat/FaliujsagElemek";
/// classes
pub const CLASSES: &str = "/ellenorzo/V3/Sajat/OsztalyCsoportok";
/// class averages
pub const CLASS_AVERAGES: &str = "/V3/Sajat/Ertekelesek/Atlagok/OsztalyAtlagok";
/// homeworks
pub const HOMEWORKS: &str = "/ellenorzo/V3/Sajat/HaziFeladatok";
/// homeworks that are done
pub const HOMEWORK_DONE: &str = "/ellenorzo/V3/Sajat/HaziFeladatok/Megoldva";
/// all poor institutes using `Kréta`
pub const INSTITUTES: &str = "/ellenorzo/V3/Sajat/Intezmenyek";

pub const SEND_MESSAGE: &str = "/api/v1/kommunikacio/uzenetek";
/// get all messages with `kind`
pub fn get_all_msgs(kind: &str) -> String {
    format!("/api/v1/kommunikacio/postaladaelemek/{kind}")
}
/// get detailed information about message with `id`
pub fn get_msg(id: u64) -> String {
    format!("/api/v1/kommunikacio/postaladaelemek/{id}")
}

/// trash message
pub const TRASH_MESSAGE: &str = "/api/v1/kommunikacio/postaladaelemek/kuka";
/// delete message
pub const DELETE_MESSAGE: &str = "/api/v1/kommunikacio/postaladaelemek/torles";

/// recipient categories
pub const RECIPIENT_CATEGORIES: &str = "/api/v1/adatszotarak/cimzetttipusok";
/// available recipient categories
pub const AVAILABLE_CATEGORIES: &str = "/api/v1/kommunikacio/cimezhetotipusok";
/// teacher recipients
pub const RECIPIENTS_TEACHER: &str = "/api/v1/kreta/alkalmazottak/tanar";

/// upload attachment
pub const UPLOAD_ATTACHMENT: &str = "/ideiglenesfajlok";
/// download attachment
pub fn download_attachment(id: u64) -> String {
    format!("/api/v1/dokumentumok/uzenetek/{id}")
}
