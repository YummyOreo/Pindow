use device_query::Keycode;

use crate::config::key;
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

fn map_defaults(keymaps: Vec<key::Keybind>) -> Vec<key::Keybind> {
    let mut keymaps_new = keymaps.clone();
    let defaults = vec![
        key::Keybind {
            keys: vec![Keycode::LControl, Keycode::Comma],
            event: key::Event::OpenApp,
        },
        key::Keybind {
            keys: vec![Keycode::LControl, Keycode::Apostrophe],
            event: key::Event::AddApp,
        },
        key::Keybind {
            keys: vec![Keycode::LControl, Keycode::Grave],
            event: key::Event::IncrementSetConfig,
        },
        key::Keybind {
            keys: vec![Keycode::RControl, Keycode::RAlt],
            event: key::Event::DebugClose,
        },
    ];
    for map in defaults {
        let mut contains = false;
        for keymap in &keymaps {
            if &map.event == &keymap.event {
                contains = true;
            }
        }
        if !contains {
            keymaps_new.push(map.clone());
        }
    }
    keymaps_new
}

fn map_keymaps(maps: Vec<options::KeybindingsStr>) -> Vec<key::Keybind> {
    let mut keymaps: Vec<key::Keybind> = vec![];
    for map in maps {
        let mut keys: Vec<Keycode> = vec![];
        for key in map.keys {
            keys.push(keycode_from_string(&key[0..key.len()]).unwrap());
        }

        for key in map.modifiers.unwrap_or(vec![]) {
            keys.push(keycode_from_string(&key).unwrap());
        }

        let event = match_event(&map.event).unwrap();
        keymaps.push(key::Keybind { keys, event })
    }
    map_defaults(keymaps)
}

fn map_app_commands(
    app_commands_str: Option<Vec<options::AppCommandStr>>,
) -> Vec<options::AppCommand> {
    let mut app_commands = vec![];
    for i in app_commands_str.unwrap_or(vec![]) {
        app_commands.push(options::AppCommand {
            app: i.app_path,
            args: i.args.unwrap_or(vec![]),
        });
    }
    app_commands
}

fn map_config(config_str: options::ConfigStr, index: i32) -> options::Config {
    options::Config {
        name: config_str.name.unwrap_or(index.to_string()),
        app_commands: map_app_commands(config_str.apps),
        timeout: config_str.timeout.unwrap_or(5) * 1000,
        keymaps: map_keymaps(config_str.keymaps.unwrap_or(vec![])),
    }
}

pub fn map_options(config_str: options::OptionsStr) -> options::Options {
    let mut configs: Vec<options::Config> = Vec::default();

    let mut index = 0;
    for config in config_str.configs {
        configs.push(map_config(config, index));
        index += 1;
    }

    let options = options::Options {
        configs,
        current_config: 0,
        args: Default::default(),
    };

    options
}

fn match_event(s: &str) -> Option<key::Event> {
    match s {
        "OpenApp" => Some(key::Event::OpenApp),
        "AddApp" => Some(key::Event::AddApp),
        "IncrementConfig" => Some(key::Event::IncementConfig),
        "DecrementConfig" => Some(key::Event::DecrementConfig),
        "IncrementSetConfig" => Some(key::Event::IncrementSetConfig),
        "DecrementSetConfig" => Some(key::Event::DecrementSetConfig),
        "DebugClose" => Some(key::Event::DebugClose),
        _ => {
            if s.starts_with("OpenApp") {
                let s = s.replace("OpenApp", "");
                Some(key::Event::OpenAppNum(
                    s.to_string().parse::<usize>().unwrap(),
                ))
            } else if s.starts_with("SetConfig") {
                let s = s.replace("SetConfig", "");
                Some(key::Event::SetConfigNum(
                    s.to_string().parse::<usize>().unwrap(),
                ))
            } else {
                None
            }
        }
    }
}

fn keycode_from_string(s: &str) -> Option<Keycode> {
    match s {
        "1" => Some(Keycode::Key1),
        "2" => Some(Keycode::Key2),
        "3" => Some(Keycode::Key3),
        "4" => Some(Keycode::Key4),
        "5" => Some(Keycode::Key5),
        "6" => Some(Keycode::Key6),
        "7" => Some(Keycode::Key7),
        "8" => Some(Keycode::Key8),
        "9" => Some(Keycode::Key9),
        "a" => Some(Keycode::A),
        "b" => Some(Keycode::B),
        "c" => Some(Keycode::C),
        "d" => Some(Keycode::D),
        "e" => Some(Keycode::E),
        "f" => Some(Keycode::F),
        "g" => Some(Keycode::G),
        "h" => Some(Keycode::H),
        "i" => Some(Keycode::I),
        "j" => Some(Keycode::J),
        "k" => Some(Keycode::K),
        "l" => Some(Keycode::L),
        "m" => Some(Keycode::M),
        "n" => Some(Keycode::N),
        "o" => Some(Keycode::O),
        "p" => Some(Keycode::P),
        "q" => Some(Keycode::Q),
        "r" => Some(Keycode::R),
        "s" => Some(Keycode::S),
        "t" => Some(Keycode::T),
        "u" => Some(Keycode::U),
        "v" => Some(Keycode::V),
        "w" => Some(Keycode::W),
        "x" => Some(Keycode::X),
        "y" => Some(Keycode::Y),
        "z" => Some(Keycode::Z),
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
        "LCtrl" => Some(Keycode::LControl),
        "RCtrl" => Some(Keycode::RControl),
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
        "`" => Some(Keycode::Grave),
        "-" => Some(Keycode::Minus),
        "=" => Some(Keycode::Equal),
        "[" => Some(Keycode::LeftBracket),
        "]" => Some(Keycode::RightBracket),
        "\\" => Some(Keycode::BackSlash),
        ";" => Some(Keycode::Semicolon),
        "'" => Some(Keycode::Apostrophe),
        "," => Some(Keycode::Comma),
        "." => Some(Keycode::Dot),
        "/" => Some(Keycode::Slash),
        _ => panic!("failed to parse keycode"),
    }
}
