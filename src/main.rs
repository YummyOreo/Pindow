use std::env;
use device_query::{DeviceQuery, DeviceState, Keycode};

mod config;
mod keybindings;
mod run;
mod window;

pub struct Arguments {
    debug: bool
}

fn get_args() -> Arguments {
    let args: Vec<String> = env::args().collect();
    let mut arguments = Arguments{debug: false};
    for arg in args {
        if arg == "--debug" || arg == "-d" {
            arguments.debug = true;
        }
    }
    return arguments;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // update the list of open processes every time someone tabs, but if they do it in a row, then
    // don't,
    // then correspond the id's to the list of apps
    // have a pointer for the processes
    // when tabbing
    // get the main window of the process (by using class name GetWindow)
    // focus it by bringing it to the top of the Z-stack
    let arguments = get_args();
    let mut user_config = config::load()?;
    user_config.debug = Some(arguments.debug);
    println!("Current Config: {}", user_config.get_current().name);

    let device_state = DeviceState::new();

    let mut key_handler = keybindings::handler::Handler {
        num: 0,
        num_time: None,
        current_keys: vec![],
        timeout: user_config.get_current().timeout,
    };
    loop {
        let keys: Vec<Keycode> = device_state.get_keys();
        let key_update: bool = key_handler.check_update(&keys);

        if key_update {
            key_handler.set_current_keys(keys.clone());
            key_handler.check_num();
            key_handler.check_num_time();

            if let Some(keybind_run) = key_handler.check_keybinds(&user_config) {
                run::run_keybind(keybind_run, &mut user_config, &key_handler);
                key_handler.reset_num();
            }
        }
    }
}
