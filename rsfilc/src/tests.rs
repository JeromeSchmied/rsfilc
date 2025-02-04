use super::paths::*;

#[test]
fn cache_path_exists() {
    assert!(cache_dir().is_some());
}
#[test]
fn config_path_exists() {
    assert!(config_path().is_some());
}
#[test]
fn cred_path_exists() {
    assert!(cred_path().is_some());
}
#[test]
/// just check whether it panics
fn dl_path_exists() {
    download_dir();
}
