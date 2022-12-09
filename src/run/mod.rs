mod application;
mod config;
mod debug;
mod utils;

use crate::config::options::Configurations;
use crate::keybindings::handler::Handler;

pub enum KeybindRun {
    RunAppNum,
    ChangeConfig,
    DebugQuit,
}

pub fn run_keybind(keybind: KeybindRun, user_configs: &mut Configurations, key_handler: &mut Handler) {
    match keybind {
        KeybindRun::RunAppNum => application::run_app(&user_configs.get_current(), key_handler),
        KeybindRun::ChangeConfig => config::change_config(user_configs, key_handler),
        KeybindRun::DebugQuit => {
            println!("quit");
            debug::quit();
        }
    }
}
