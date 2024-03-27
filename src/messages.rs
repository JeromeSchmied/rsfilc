/// kinds of message
pub enum MessageKind {
    Beerkezett,
    Elkuldott,
    Torolt,
}
impl MessageKind {
    pub fn val(&self) -> String {
        match self {
            MessageKind::Beerkezett => "beerkezett".to_owned(),
            MessageKind::Elkuldott => "elkuldott".to_owned(),
            MessageKind::Torolt => "torolt".to_owned(),
        }
    }
}
