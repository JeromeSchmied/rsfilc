pub const SEND_MESSAGE: &str = "/api/v1/kommunikacio/uzenetek";
pub fn get_all_messages(endpoint: &str) -> String {
    format!("/api/v1/kommunikacio/postaladaelemek/{endpoint}")
}
pub fn get_msg(id: &str) -> String {
    format!("/api/v1/kommunikacio/postaladaelemek/{id}")
}
pub const RECIPIENT_CATEGORIES: &str = "/api/v1/adatszotarak/cimzetttipusok";
pub const AVAILABLE_CATEGORIES: &str = "/api/v1/kommunikacio/cimezhetotipusok";
pub const RECIPIENTS_TEACHER: &str = "/api/v1/kreta/alkalmazottak/tanar";
pub const UPLOAD_ATTACHMENT: &str = "/ideiglenesfajlok";
pub fn download_attachment(id: &str) -> String {
    format!("/v1/dokumentumok/uzenetek/{id}")
}
pub const TRASH_MESSAGE: &str = "/api/v1/kommunikacio/postaladaelemek/kuka";
pub const DELETE_MESSAGE: &str = "/api/v1/kommunikacio/postaladaelemek/torles";
