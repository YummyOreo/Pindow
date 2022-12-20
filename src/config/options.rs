use serde::{Deserialize, Serialize};

use crate::arguments::Arguments;
use crate::config::key;
use crate::error;

#[derive(Debug, Clone, Default)]
pub struct AppCommand {
    pub app: String,
    pub args: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub name: String,
    pub app_commands: Vec<AppCommand>,
    pub timeout: u128,
    pub keymaps: Vec<key::Keybind>,
}

#[derive(Debug, Clone)]
pub struct Options {
    pub configs: Vec<Config>,
    pub current_config: usize,
    pub args: Arguments,
}

impl Options {
    pub fn get_current(&self) -> Config {
        self.configs[self.current_config].clone()
    }

    pub fn set_current(&mut self, index: usize) -> Result<Config, error::config::SetConfigError> {
        if index >= self.configs.len() {
            Err(error::config::SetConfigError {
                num: (index + 1) as i32,
            })
        } else {
            self.current_config = index;
            Ok(self.get_current())
        }
    }

    pub fn increment(&mut self) {
        self.current_config += 1;
        if self.current_config >= self.configs.len() {
            self.current_config = 0;
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct KeybindingsStr {
    pub keys: Vec<String>,
    pub modifiers: Option<Vec<String>>,
    pub event: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct AppCommandStr {
    pub app_path: String,
    pub args: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct ConfigStr {
    pub name: Option<String>,
    pub apps: Option<Vec<AppCommandStr>>,
    pub timeout: Option<u128>,
    pub keymaps: Option<Vec<KeybindingsStr>>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct OptionsStr {
    pub configs: Vec<ConfigStr>,
    pub timeout: Option<u128>,
    pub keymaps: Option<Vec<KeybindingsStr>>
}
