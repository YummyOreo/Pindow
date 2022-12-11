use device_query::Keycode;
use serde::Deserialize;

use crate::error;

#[derive(Debug, Clone, Default)]
pub struct Keybindings {
    pub app_num: Vec<Keycode>,
    pub tab_app: Vec<Keycode>,
    pub change_config: Vec<Keycode>,
    pub debug_close: Vec<Keycode>,
}

#[derive(Debug, Clone, Default)]
pub struct AppCommand {
    pub app: String,
    pub args: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct Config {
    pub name: String,
    pub app_commands: Vec<AppCommand>,
    pub timeout: u128,
    pub key_bindings: Keybindings,
}

#[derive(Debug, Clone, Default)]
pub struct Args {
    pub debug: Option<bool>,
    pub start_config: Option<usize>,
    pub path: Option<String>,
    pub help: Option<bool>,
    pub get_path: Option<bool>
}

#[derive(Debug, Clone)]
pub struct Configurations {
    pub configs: Vec<Config>,
    pub current_config: usize,
    pub args: Args,
}

impl Configurations {
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

#[derive(Debug, Deserialize, Clone)]
pub struct KeybindingsStr {
    pub app_num: Option<Vec<String>>,
    pub tab_app: Option<Vec<String>>,
    pub change_config: Option<Vec<String>>,
    pub debug_close: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppCommandStr {
    pub app_path: String,
    pub args: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ConfigStr {
    pub name: Option<String>,
    pub apps: Option<Vec<AppCommandStr>>,
    pub timeout: Option<u128>,
    pub keybindings: Option<KeybindingsStr>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ConfigurationsStr {
    pub configs: Vec<ConfigStr>,
}
