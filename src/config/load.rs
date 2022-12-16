use device_query::Keycode;

use crate::config::options;
use crate::config::key;

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

fn map_keymaps(maps: Option<options::KeybindingsStr>) -> Vec<key::Keybind> {
    vec![]
}

fn map_app_commands(app_commands_str: Option<Vec<options::AppCommandStr>>) -> Vec<options::AppCommand> {
    match app_commands_str {
        Some(apps) => {
            let mut app_commands = vec![];
            for i in apps {
                app_commands.push(options::AppCommand{
                    app: i.app_path,
                    args: i.args.unwrap_or(vec![])
                });
            }
            app_commands
        },
        None => vec![]
    }
}

fn map_config(config_str: options::ConfigStr, index: i32) -> options::Config {
    options::Config {
        name: config_str.name.unwrap_or(index.to_string()),
        app_commands: map_app_commands(config_str.apps),
        timeout: config_str.timeout.unwrap_or(5),
        keymaps: map_keymaps(config_str.keymaps),
    }
}

pub fn map_options(config_str: options::OptionsStr) -> options::Options {
    let mut configs: Vec<options::Config> = Vec::default();
    let options = options::Options {
        configs,
        current_config: 0,
        args: Default::default(),
    };

    for i in config_str.configs {
    }
    options
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
        "~" => Some(Keycode::Grave),
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
