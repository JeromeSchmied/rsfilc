use super::*;

#[test]
fn en_then_decode() {
    let orig_passwd = "it's_magic×2004.12.30";
    let usr = User::new("simon", orig_passwd, "klik00000000");
    let encoded_passwd = &usr.password;

    assert_ne!(orig_passwd, encoded_passwd);
    assert_eq!(orig_passwd, usr.decode_password());
}

#[test]
fn ser_user() {
    let user = User::new("Test Paul", "2000.01.01", "klik0000001");

    let user_toml = "\
username = \"Test Paul\"
password = \"MjAwMC4wMS4wMQ==\"
school_id = \"klik0000001\"
";

    assert_eq!(Ok(user_toml.to_owned()), toml::to_string(&user));
}

#[test]
fn deser_user() {
    let user = toml::from_str(
        r#"
            username = "Test Paul"
            password = "MjAwMC4wMS4wMQ=="
            school_id = "klik0000001"
            "#,
    );
    assert_eq!(
        Ok(User::new("Test Paul", "2000.01.01", "klik0000001")),
        user
    );
}

#[test]
fn ser_users() {
    let users: Users = vec![
        User::new("Test Paul", "2000.01.01", "klik0000001"),
        User::new("Test Paulina", "2000.01.02", "klik0000002"),
    ]
    .into();

    let user_toml = r#"[[users]]
username = "Test Paul"
password = "MjAwMC4wMS4wMQ=="
school_id = "klik0000001"

[[users]]
username = "Test Paulina"
password = "MjAwMC4wMS4wMg=="
school_id = "klik0000002"
"#;

    assert_eq!(Ok(user_toml.to_owned()), toml::to_string(&users));
}

#[test]
fn deser_users() {
    let users: Users = vec![
        User::new("Test Paul", "2000.01.01", "klik0000001"),
        User::new("Test Paulina", "2000.01.02", "klik0000002"),
    ]
    .into();

    let user_toml = r#"[[users]]
username = "Test Paul"
password = "MjAwMC4wMS4wMQ=="
school_id = "klik0000001"

[[users]]
username = "Test Paulina"
password = "MjAwMC4wMS4wMg=="
school_id = "klik0000002"
"#;

    assert_eq!(toml::to_string(&users), Ok(user_toml.to_owned()));
}

#[test]
fn config_ser() {
    let config = Config {
        default_username: "Me Me Me!".to_owned(),
    };
    let config_toml = r#"default_username = "Me Me Me!"
"#;
    assert_eq!(Ok(config_toml.to_owned()), toml::to_string(&config));
}
#[test]
fn config_deser() {
    let config_toml = r#"default_username = "Me Me Me!"
"#;
    let config = Config {
        default_username: "Me Me Me!".to_owned(),
    };
    assert_eq!(toml::from_str(config_toml), Ok(config));
}
