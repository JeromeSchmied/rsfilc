//! basic User info, `Kréta` stores

use ekreta::UserInfo;
use std::fmt::Write;

pub fn disp(user_info: &UserInfo) -> String {
    let mut f = String::new();
    _ = writeln!(&mut f, "| {}", user_info.nev);
    _ = writeln!(&mut f, "| Intézmény: {}", user_info.intezmeny_nev);
    _ = writeln!(&mut f, "|   id: {}", user_info.intezmeny_azonosito);
    _ = write!(
        &mut f,
        "| Születési dátum: {}",
        user_info.szuletesi_datum.format("%Y.%m.%d")
    );
    f
}
