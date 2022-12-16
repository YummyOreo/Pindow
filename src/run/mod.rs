mod application;
mod config;
mod debug;
mod utils;

use crate::config::key::Event;
use crate::config::options::Options;
use crate::keybindings::handler::Handler;

pub fn run_keybind(keymap: Event, user_configs: &mut Options, key_handler: &mut Handler) {
    match keymap {
        Event::OpenApp => application::run_app(&user_configs.get_current(), key_handler),
        Event::AddApp => application::add_config(user_configs, key_handler),
        Event::IncrementSetConfig => config::change_config(user_configs, key_handler),
        Event::DebugClose => {
            if user_configs.args.debug {
                println!("Quitting...");
                debug::quit();
            }
        }
        _ => {}
    }
}
