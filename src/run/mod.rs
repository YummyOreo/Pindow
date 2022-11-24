mod application;
mod debug;

use crate::config::options::Config;
use crate::keybindings::handler::Handler;

pub enum KeybindRun {
    RunAppNum,
    DebugQuit,
}

pub fn run_keybind(keybind: KeybindRun, user_config: &Config, key_handler: &Handler) {
    match keybind {
        KeybindRun::RunAppNum => application::run_app(user_config, key_handler),
        KeybindRun::DebugQuit => {
            debug::quit();
        }
    }
}
