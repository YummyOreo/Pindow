use device_query::{DeviceQuery, DeviceState, Keycode};

mod config;
mod keybindings;
mod run;
mod window;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Update the processes ever second or so, or if you spawn a new one (keybind to do so)
    // save each program that corresponds to a program to a struct.
    // Each item is a program with the items:
    // {
    // Vec<HWND NUM>
    // Pointer: num (for vec)
    // }
    // This only gets updated if there is a change, and keeps it position (ie if something is
    // added, then it goes to the back, if something is removed then it gets removed. But it should
    // not move)
    // windows::test();
    let user_config = config::load()?;
    let apps_windows =
        vec![window::handler::ApplicationWindows::new(); user_config.app_commands.len()];
    let current_windows: Vec<isize> = window::utils::win::get_windows();
    // println!("{:?}", current_windows);
    for mut i in apps_windows {
        i.update(&current_windows);
    }
    let device_state = DeviceState::new();

    let mut key_handler = keybindings::handler::Handler {
        num: 0,
        num_time: None,
        current_keys: vec![],
        timeout: user_config.timeout,
    };
    loop {
        let keys: Vec<Keycode> = device_state.get_keys();
        let key_update: bool = key_handler.check_update(&keys);

        if key_update {
            key_handler.set_current_keys(keys.clone());
            key_handler.check_num();
            key_handler.check_num_time();

            if let Some(keybind_run) = key_handler.check_keybinds(&user_config) {
                run::run_keybind(keybind_run, &user_config, &key_handler);
                key_handler.reset_num();
            }
        }
    }
}
