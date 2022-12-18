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
        Event::OpenAppNum(n) => application::run_app_by_num(&user_configs.get_current(), n),

        Event::AddApp => application::add_config(user_configs, key_handler),

        Event::IncementConfig => config::incement_config(user_configs, key_handler),
        Event::IncrementSetConfig => config::inc_set_config(user_configs, key_handler),

        Event::DecrementConfig => config::decrement_config(user_configs, key_handler),
        Event::DecrementSetConfig => config::dec_set_config(user_configs, key_handler),

        Event::SetConfigNum(n) => config::set_config(user_configs, key_handler, n - 1),

        Event::DebugClose => {
            if user_configs.args.debug {
                debug::quit();
            }
        }
    }
}
