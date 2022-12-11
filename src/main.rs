use device_query::{DeviceQuery, DeviceState, Keycode};

mod arguments;
mod config;
mod error;
mod keybindings;
mod run;
mod window;
mod info;

fn load_current_config() -> config::options::Configurations {
    let arguments = arguments::get_args();
    let mut user_config = config::load(arguments.path.clone());

    user_config.args = config::options::Args {
        debug: Some(arguments.debug),
        start_config: arguments.start_config,
        path: arguments.path,
        help: Some(arguments.help),
        get_path: Some(arguments.get_path)
    };
    if let Some(current_config) = user_config.args.start_config {
        user_config.set_current(current_config).unwrap();
    }

    user_config
}

fn check_info(args: config::options::Args) {
    match args.help {
        Some(true) => info::help::print_help_menue(),
        _ => {}
    }
    match args.get_path {
        Some(true) => info::help::print_config_path(),
        _ => {}
    }
}

fn get_key_handler(timeout: u128) -> keybindings::handler::Handler {
    keybindings::handler::Handler {
        timeout,
        ..Default::default()
    }
}

fn keybinding_update(
    keys: Vec<Keycode>,
    key_handler: &mut keybindings::handler::Handler,
    user_config: &mut config::options::Configurations,
) {
    key_handler.set_current_keys(keys.clone());
    key_handler.check_num();
    key_handler.check_num_time();

    if let Some(keybind_run) = key_handler.check_keybinds(&user_config) {
        run::run_keybind(keybind_run, user_config, key_handler);
        key_handler.reset_num();
    }
}

fn main_loop(
    user_config: &mut config::options::Configurations,
    key_handler: &mut keybindings::handler::Handler,
) {
    let device_state = DeviceState::new();

    loop {
        let keys: Vec<Keycode> = device_state.get_keys();
        let key_update: bool = key_handler.check_update(&keys);

        if key_update {
            keybinding_update(keys, key_handler, user_config);
        }
    }
}

fn main() {
    // update the list of open processes every time someone tabs, but if they do it in a row, then
    // don't,
    // then correspond the id's to the list of apps
    // have a pointer for the processes
    // when tabbing
    // get the main window of the process (by using class name GetWindow)
    // focus it by bringing it to the top of the Z-stack
    let mut user_config = load_current_config();

    check_info(user_config.args.clone());

    println!("Current Config: {}", user_config.get_current().name);

    let mut key_handler = get_key_handler(user_config.get_current().timeout);

    main_loop(&mut user_config, &mut key_handler);
}
