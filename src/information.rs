//! basic User info, `Kréta` stores

use ekreta::{Res, UserInfo};

pub fn display(conf: &crate::Config) -> Res<()> {
    let mut table = ascii_table::AsciiTable::default();
    let headers = ["NÉV", "OM AZONOSÍTÓ", "OSKOLA", "SZÜLETETT"];
    for (i, head) in headers.into_iter().enumerate() {
        table.column(i).set_header(head);
    }
    let data = conf
        .users
        .iter()
        .map(|u| {
            let info = u.get_userinfo()?;
            Ok(disp(&info, &u.0.username, &conf.default_username))
        })
        .collect::<Res<Vec<_>>>()?;
    table.print(data);

    Ok(())
}

fn disp(user_info: &UserInfo, id: &str, def_id: &str) -> Vec<String> {
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
