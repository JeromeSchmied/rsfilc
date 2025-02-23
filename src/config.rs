use crate::{Res, Usr};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeSet, path::PathBuf};

pub const APP_NAME: &str = "rsfilc";
const CONFIG_NAME: &str = "config";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Config {
    pub default_username: String,
    pub users: BTreeSet<Usr>,
}
impl Config {
    pub fn load() -> Res<Config> {
        Ok(confy::load(APP_NAME, CONFIG_NAME)?)
    }
    pub fn save(&self) -> Res<()> {
        Ok(confy::store(APP_NAME, CONFIG_NAME, self)?)
    }
    pub fn switch_user_to(&mut self, name: impl ToString) {
        self.default_username = name.to_string();
    }
    pub fn delete(&mut self, name: impl AsRef<str>) {
        self.users.retain(|usr| usr.0.username != name.as_ref());
        if self.default_username == name.as_ref() {
            let _ = crate::cache::delete_dir(name.as_ref());
            // set default to the first element, not to die
            if let Some(first) = self.users.first().cloned() {
                self.switch_user_to(first.0.username);
            } else {
                self.switch_user_to(String::new());
            }
        }
    }
    /// checks if the given `name` (either userid or username) is saved in conf and returns the userid
    pub fn get_userid(&self, name: impl AsRef<str>) -> Option<String> {
        self.users
            .iter()
            .find(|user| {
                user.0.username == name.as_ref()
                    || user.get_userinfo().is_ok_and(|ui| {
                        ui.nev
                            .to_lowercase()
                            .contains(&name.as_ref().to_lowercase())
                    })
            })
            .map(|u| u.0.username.clone())
    }
    pub fn path() -> Res<PathBuf> {
        Ok(confy::get_configuration_file_path(APP_NAME, CONFIG_NAME)?)
    }
}
