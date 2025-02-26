//! basic User info, `Kréta` stores

use crate::user::Usr;
use ekreta::{Res, UserInfo};

pub fn disp<'a>(users: impl Iterator<Item = &'a Usr>, def_id: &str) -> Res<()> {
    let mut table = ascii_table::AsciiTable::default();
    let headers = ["NÉV", "OM AZONOSÍTÓ", "OSKOLA", "SZÜLETETT"];
    for (i, head) in headers.into_iter().enumerate() {
        table.column(i).set_header(head);
    }
    let data = users
        .into_iter()
        .map(|u| {
            let info = u.get_userinfo()?;
            Ok(_disp(&info, &u.0.username, def_id))
        })
        .collect::<Res<Vec<_>>>()?;
    table.print(data);

    Ok(())
}

pub fn _disp(user_info: &UserInfo, id: &str, def_id: &str) -> Vec<String> {
    let def_usr = if id == def_id {
        "Alapértelmezett: "
    } else {
        ""
    };
    let name = format!("{def_usr}{}", user_info.nev);
    let id = id.to_string();
    let school = user_info.intezmeny_nev.clone();
    let birth = user_info.szuletesi_datum.format("%Y.%m.%d").to_string();
    vec![name, id, school, birth]
}
