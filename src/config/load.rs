use device_query::Keycode;

use crate::config::options;

pub fn load_string(file: String) -> String {
    match std::fs::read_to_string(&file) {
        Ok(data) => data,
        _ => {
            make_file(&file);
            load_string(file)
        }
    }
}

pub fn make_file(file: &String) {
    let path = std::path::Path::new(file);
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).unwrap();
    std::fs::write(file, "{\"configs\": [{}]}").expect("Unable to write file");
}

fn map_keybindings(keybindings_str: options::KeybindingsStr) -> options::Keybindings {
    let mut debug_close: Vec<Keycode>;
    match keybindings_str.debug_close {
        None => {
            debug_close = vec![Keycode::RControl, Keycode::RAlt];
        }
        Some(s) => {
            debug_close = Vec::new();
            for key in s {
                debug_close.insert(0, keycode_from_string(&key).unwrap());
            }
        }
    }

    let mut app_num: Vec<Keycode>;
    match keybindings_str.app_num {
        None => {
            app_num = vec![Keycode::LControl, Keycode::Comma];
        }
        Some(s) => {
            app_num = Vec::new();
            for key in s {
                app_num.insert(0, keycode_from_string(&key).unwrap());
            }
        }
    }

    let mut change_config: Vec<Keycode>;
    match keybindings_str.change_config {
        None => {
            change_config = vec![Keycode::LControl, Keycode::Grave];
        }
        Some(s) => {
            change_config = Vec::new();
            for key in s {
                change_config.insert(0, keycode_from_string(&key).unwrap());
            }
        }
    };

    let mut tab_app: Vec<Keycode>;
    match keybindings_str.tab_app {
        None => {
            tab_app = vec![Keycode::LControl, Keycode::Apostrophe];
        }
        Some(s) => {
            tab_app = Vec::new();
            for key in s {
                tab_app.insert(0, keycode_from_string(&key).unwrap());
            }
        }
    };

    options::Keybindings {
        app_num,
        tab_app,
        change_config,
        debug_close,
    }
}

pub fn map_config(options: options::ConfigStr, index: usize) -> options::Config {
    let keybindings: options::KeybindingsStr;
    match options.keybindings {
        None => {
            keybindings = options::KeybindingsStr {
                app_num: None,
                tab_app: None,
                change_config: None,
                debug_close: None,
            };
        }
        Some(s) => {
            keybindings = s;
        }
    }

    let mut app_commands: Vec<options::AppCommand>;
    match options.apps {
        None => {
            app_commands = Default::default();
        }
        Some(s) => {
            let app_commands_str = s.to_vec();
            app_commands = Vec::new();
            for command in app_commands_str.into_iter() {
                let mut app_command: options::AppCommand = Default::default();
                app_command.app = command.app_path;
                if let Some(args) = command.args {
                    app_command.args = args;
                }
                app_commands.insert(app_commands.len(), app_command);
            }
        }
    }

    let timeout: u128;
    match options.timeout {
        None => {
            timeout = 5 * 1000;
        }
        Some(s) => {
            timeout = s * 1000;
        }
    }

    let name: String;
    match options.name {
        None => {
            name = index.to_string();
        }
        Some(s) => {
            name = s;
        }
    }

    options::Config {
        name,
        app_commands,
        timeout,
        key_bindings: map_keybindings(keybindings),
    }
}

pub fn map_configurations(config: options::ConfigurationsStr) -> options::Configurations {
    let mut configs: Vec<options::Config> = vec![];
    for i in config.configs {
        configs.insert(configs.len(), map_config(i, configs.len()));
    }
    options::Configurations {
        configs,
        current_config: 0,
        args: options::Args {
            ..Default::default()
        },
    }
}

fn keycode_from_string(s: &str) -> Option<Keycode> {
    match s {
        "Key1" => Some(Keycode::Key1),
        "Key2" => Some(Keycode::Key2),
        "Key3" => Some(Keycode::Key3),
        "Key4" => Some(Keycode::Key4),
        "Key5" => Some(Keycode::Key5),
        "Key6" => Some(Keycode::Key6),
        "Key7" => Some(Keycode::Key7),
        "Key8" => Some(Keycode::Key8),
        "Key9" => Some(Keycode::Key9),
        "A" => Some(Keycode::A),
        "B" => Some(Keycode::B),
        "C" => Some(Keycode::C),
        "D" => Some(Keycode::D),
        "E" => Some(Keycode::E),
        "F" => Some(Keycode::F),
        "G" => Some(Keycode::G),
        "H" => Some(Keycode::H),
        "I" => Some(Keycode::I),
        "J" => Some(Keycode::J),
        "K" => Some(Keycode::K),
        "L" => Some(Keycode::L),
        "M" => Some(Keycode::M),
        "N" => Some(Keycode::N),
        "O" => Some(Keycode::O),
        "P" => Some(Keycode::P),
        "Q" => Some(Keycode::Q),
        "R" => Some(Keycode::R),
        "S" => Some(Keycode::S),
        "T" => Some(Keycode::T),
        "U" => Some(Keycode::U),
        "V" => Some(Keycode::V),
        "W" => Some(Keycode::W),
        "X" => Some(Keycode::X),
        "Y" => Some(Keycode::Y),
        "Z" => Some(Keycode::Z),
        "F1" => Some(Keycode::F1),
        "F2" => Some(Keycode::F2),
        "F3" => Some(Keycode::F3),
        "F4" => Some(Keycode::F4),
        "F5" => Some(Keycode::F5),
        "F6" => Some(Keycode::F6),
        "F7" => Some(Keycode::F7),
        "F8" => Some(Keycode::F8),
        "F9" => Some(Keycode::F9),
        "F10" => Some(Keycode::F10),
        "F11" => Some(Keycode::F11),
        "F12" => Some(Keycode::F12),
        "Escape" => Some(Keycode::Escape),
        "Space" => Some(Keycode::Space),
        "LControl" => Some(Keycode::LControl),
        "RControl" => Some(Keycode::RControl),
        "LShift" => Some(Keycode::LShift),
        "RShift" => Some(Keycode::RShift),
        "LAlt" => Some(Keycode::LAlt),
        "RAlt" => Some(Keycode::RAlt),
        "Meta" => Some(Keycode::Meta),
        "Enter" => Some(Keycode::Enter),
        "Up" => Some(Keycode::Up),
        "Down" => Some(Keycode::Down),
        "Left" => Some(Keycode::Left),
        "Right" => Some(Keycode::Right),
        "Backspace" => Some(Keycode::Backspace),
        "CapsLock" => Some(Keycode::CapsLock),
        "Tab" => Some(Keycode::Tab),
        "Home" => Some(Keycode::Home),
        "End" => Some(Keycode::End),
        "PageUp" => Some(Keycode::PageUp),
        "PageDown" => Some(Keycode::PageDown),
        "Insert" => Some(Keycode::Insert),
        "Delete" => Some(Keycode::Delete),
        "Grave" => Some(Keycode::Grave),
        "Minus" => Some(Keycode::Minus),
        "Equal" => Some(Keycode::Equal),
        "LeftBracket" => Some(Keycode::LeftBracket),
        "RightBracket" => Some(Keycode::RightBracket),
        "BackSlash" => Some(Keycode::BackSlash),
        "Semicolon" => Some(Keycode::Semicolon),
        "Apostrophe" => Some(Keycode::Apostrophe),
        "Comma" => Some(Keycode::Comma),
        "Dot" => Some(Keycode::Dot),
        "Slash" => Some(Keycode::Slash),
        _ => panic!("failed to parse keycode"),
    }
}
