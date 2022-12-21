mod application;
mod config;
mod debug;
mod utils;

use crate::config::key::Event;
use crate::config::options::Options;
use crate::error::run;
use crate::keybindings::handler::Handler;

pub fn run_keybind(
    keymap: &Event,
    user_configs: &mut Options,
    key_handler: &mut Handler,
) -> Result<(), run::RunEventError> {
    let mut result: Option<()> = None;
    match keymap {
        Event::OpenApp => result = application::run_app(&user_configs.get_current(), key_handler),
        Event::OpenAppNum(n) => {
            result = application::run_app_by_num(&user_configs.get_current(), n - 1)
        }

        Event::AddApp => result = application::add_config(user_configs, key_handler),

        Event::IncementConfig => result = config::incement_config(user_configs, key_handler),
        Event::IncrementSetConfig => result = config::inc_set_config(user_configs, key_handler),

        Event::DecrementConfig => result = config::decrement_config(user_configs, key_handler),
        Event::DecrementSetConfig => result = config::dec_set_config(user_configs, key_handler),

        Event::SetConfigNum(n) => result = config::set_config(user_configs, key_handler, n - 1),

        Event::DebugClose => {
            if user_configs.args.debug {
                debug::quit();
            }
        }
    }

    match result {
        None => Err(run::RunEventError { event: keymap.clone() }),
        Some(_) => Ok(()),
    }
}
