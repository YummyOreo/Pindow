use device_query::Keycode;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Keybindings {
    pub app_num: Vec<Keycode>,
    pub debug_close: Vec<Keycode>,
}

#[derive(Debug, Clone, Default)]
pub struct AppCommand {
    pub app: String,
    pub args: Vec<String>,
}

#[derive(Debug)]
pub struct Config {
    pub app_commands: Vec<AppCommand>,
    pub timeout: u128,
    pub key_bindings: Keybindings,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KeybindingsStr {
    pub app_num: Option<Vec<String>>,
    pub debug_close: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppCommandStr {
    pub app: String,
    pub args: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConfigStr {
    pub app_commands: Option<Vec<AppCommandStr>>,
    pub timeout: Option<u128>,
    pub keybindings: Option<KeybindingsStr>,
}
