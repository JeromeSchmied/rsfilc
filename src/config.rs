use crate::{Res, Usr};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

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
    pub fn switch_user_to(&mut self, name: String) -> Res<()> {
        if self.users.iter().any(|u| u.0.username == name) {
            self.default_username = name;
            Ok(())
        } else {
            Err(format!("'{name}' doesn't exist, check out `rsfilc user`").into())
        }
    }
    pub fn delete(&mut self, name: &str) {
        self.users.retain(|usr| usr.0.username != name);
        if self.default_username == name {
            let _ = crate::cache::delete_dir(name);
            // set default to the first element, not to die
            if let Some(first) = self.users.first().cloned() {
                _ = self.switch_user_to(first.0.username);
            } else {
                _ = self.switch_user_to(String::new());
            }
        }
    }
}
