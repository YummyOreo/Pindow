use device_query::Keycode;

use crate::config::key;
use crate::config::options;
use crate::error::config::LoadConfigError;

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

#[derive(Debug)]
struct Defaults {
    pub timeout: u128,
    pub keymaps: Vec<key::Keybind>,
}

impl Defaults {
    pub fn new(timeout: Option<u128>, keymaps: Option<Vec<options::KeybindingsStr>>) -> Self {
        let default_keymaps = vec![
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
        let keymaps = map_keymaps(keymaps.unwrap_or(vec![]), default_keymaps);

        let timeout = timeout.unwrap_or(5);
        Defaults { timeout, keymaps }
    }
}

fn map_default_keymaps(keymaps: Vec<key::Keybind>, default_maps: Vec<key::Keybind>) -> Vec<key::Keybind> {
    let mut keymaps_new = keymaps.clone();
    for map in default_maps {
        let mut contains = false;
        for keymap in &keymaps {
            contains = &map.event != &keymap.event;
            println!("{contains}");
            println!("{keymap:?}");
        }
        if !contains {
            keymaps_new.push(map.clone());
        }
    }
    keymaps_new
}

fn map_keymaps(maps: Vec<options::KeybindingsStr>, defaults: Vec<key::Keybind>) -> Vec<key::Keybind> {
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
    map_default_keymaps(keymaps, defaults)
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

fn map_config(config_str: options::ConfigStr, defaults: &Defaults, index: i32) -> options::Config {

    options::Config {
        name: config_str.name.unwrap_or(index.to_string()),
        app_commands: map_app_commands(config_str.apps),
        timeout: config_str.timeout.unwrap_or(defaults.timeout) * 1000,
        keymaps: map_keymaps(config_str.keymaps.unwrap_or(vec![]), defaults.keymaps.clone()),
    }
}

pub fn map_options(config_str: options::OptionsStr) -> options::Options {
    let defaults = Defaults::new(config_str.timeout, config_str.keymaps.clone());
    println!("{defaults:?}");

    let mut configs: Vec<options::Config> = Vec::default();

    let mut index = 0;
    for config in config_str.configs {
        configs.push(map_config(config, &defaults, index));
        index += 1;
    }

    let options = options::Options {
        configs,
        current_config: 0,
        args: Default::default(),
    };

    options
}

fn match_event_num(s: &str) -> Result<key::Event, LoadConfigError> {
    if s.starts_with("OpenApp") {
        let s = s.replace("OpenApp", "").to_string();
        let num = s.parse::<usize>();

        if let Ok(num) = num {
            if num == 0 || num > 9 {
                return Err(LoadConfigError::InvalidNumber(s));
            }
            return Ok(key::Event::OpenAppNum(num));
        }
        Err(LoadConfigError::InvalidNumber(s))
    } else if s.starts_with("SetConfig") {
        let s = s.replace("SetConfig", "").to_string();
        let num = s.parse::<usize>();

        if let Ok(num) = num {
            if num == 0 || num > 9 {
                return Err(LoadConfigError::InvalidNumber(s));
            }
            return Ok(key::Event::SetConfigNum(num));
        }
        Err(LoadConfigError::InvalidNumber(s))
    } else {
        Err(LoadConfigError::StringToEventError(s.to_string()))
    }
}

fn match_event(s: &str) -> Result<key::Event, LoadConfigError> {
    match s {
        "OpenApp" => Ok(key::Event::OpenApp),
        "AddApp" => Ok(key::Event::AddApp),
        "IncrementConfig" => Ok(key::Event::IncementConfig),
        "DecrementConfig" => Ok(key::Event::DecrementConfig),
        "IncrementSetConfig" => Ok(key::Event::IncrementSetConfig),
        "DecrementSetConfig" => Ok(key::Event::DecrementSetConfig),
        "DebugClose" => Ok(key::Event::DebugClose),
        _ => match_event_num(s),
    }
}

fn keycode_from_string(s: &str) -> Result<Keycode, LoadConfigError> {
    match s {
        "1" => Ok(Keycode::Key1),
        "2" => Ok(Keycode::Key2),
        "3" => Ok(Keycode::Key3),
        "4" => Ok(Keycode::Key4),
        "5" => Ok(Keycode::Key5),
        "6" => Ok(Keycode::Key6),
        "7" => Ok(Keycode::Key7),
        "8" => Ok(Keycode::Key8),
        "9" => Ok(Keycode::Key9),
        "a" => Ok(Keycode::A),
        "b" => Ok(Keycode::B),
        "c" => Ok(Keycode::C),
        "d" => Ok(Keycode::D),
        "e" => Ok(Keycode::E),
        "f" => Ok(Keycode::F),
        "g" => Ok(Keycode::G),
        "h" => Ok(Keycode::H),
        "i" => Ok(Keycode::I),
        "j" => Ok(Keycode::J),
        "k" => Ok(Keycode::K),
        "l" => Ok(Keycode::L),
        "m" => Ok(Keycode::M),
        "n" => Ok(Keycode::N),
        "o" => Ok(Keycode::O),
        "p" => Ok(Keycode::P),
        "q" => Ok(Keycode::Q),
        "r" => Ok(Keycode::R),
        "s" => Ok(Keycode::S),
        "t" => Ok(Keycode::T),
        "u" => Ok(Keycode::U),
        "v" => Ok(Keycode::V),
        "w" => Ok(Keycode::W),
        "x" => Ok(Keycode::X),
        "y" => Ok(Keycode::Y),
        "z" => Ok(Keycode::Z),
        "F1" => Ok(Keycode::F1),
        "F2" => Ok(Keycode::F2),
        "F3" => Ok(Keycode::F3),
        "F4" => Ok(Keycode::F4),
        "F5" => Ok(Keycode::F5),
        "F6" => Ok(Keycode::F6),
        "F7" => Ok(Keycode::F7),
        "F8" => Ok(Keycode::F8),
        "F9" => Ok(Keycode::F9),
        "F10" => Ok(Keycode::F10),
        "F11" => Ok(Keycode::F11),
        "F12" => Ok(Keycode::F12),
        "Escape" => Ok(Keycode::Escape),
        "Space" => Ok(Keycode::Space),
        "LCtrl" => Ok(Keycode::LControl),
        "RCtrl" => Ok(Keycode::RControl),
        "LShift" => Ok(Keycode::LShift),
        "RShift" => Ok(Keycode::RShift),
        "LAlt" => Ok(Keycode::LAlt),
        "RAlt" => Ok(Keycode::RAlt),
        "Meta" => Ok(Keycode::Meta),
        "Enter" => Ok(Keycode::Enter),
        "Up" => Ok(Keycode::Up),
        "Down" => Ok(Keycode::Down),
        "Left" => Ok(Keycode::Left),
        "Right" => Ok(Keycode::Right),
        "Backspace" => Ok(Keycode::Backspace),
        "CapsLock" => Ok(Keycode::CapsLock),
        "Tab" => Ok(Keycode::Tab),
        "Home" => Ok(Keycode::Home),
        "End" => Ok(Keycode::End),
        "PageUp" => Ok(Keycode::PageUp),
        "PageDown" => Ok(Keycode::PageDown),
        "Insert" => Ok(Keycode::Insert),
        "Delete" => Ok(Keycode::Delete),
        "`" => Ok(Keycode::Grave),
        "-" => Ok(Keycode::Minus),
        "=" => Ok(Keycode::Equal),
        "[" => Ok(Keycode::LeftBracket),
        "]" => Ok(Keycode::RightBracket),
        "\\" => Ok(Keycode::BackSlash),
        ";" => Ok(Keycode::Semicolon),
        "'" => Ok(Keycode::Apostrophe),
        "," => Ok(Keycode::Comma),
        "." => Ok(Keycode::Dot),
        "/" => Ok(Keycode::Slash),
        _ => Err(LoadConfigError::StringToKeycodeError(s.to_string())),
    }
}
