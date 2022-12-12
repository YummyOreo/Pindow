use directories::BaseDirs;
use std::process::Command;
use std::thread;
use serde_json;

use crate::config::options::{Config, Configurations};
use crate::keybindings::handler::Handler;
use crate::run::utils;
use crate::window::utils::win;
use crate::config;

pub fn run_app(user_config: &Config, key_handler: &Handler) {
    let mut num = TryInto::<usize>::try_into(key_handler.num).unwrap();

    if num > 9 {
        return;
    }

    if num == 0 {
        if let Some(res) =
            utils::get_app_by_id(user_config, win::get_id(win::current_window()) as isize)
        {
            num = res + 1;
        } else {
            return;
        }
    }

    if user_config.app_commands.len() >= num {
        let command = user_config.app_commands.clone().into_iter().nth(num - 1);

        let _ = thread::spawn(move || {
            let command_unwraped = command.unwrap();
            let mut sys_command = Command::new(command_unwraped.app);
            for arg in command_unwraped.args {
                sys_command.arg(arg);
            }
            sys_command.spawn().expect("failed to execute process");
        });
    }
}

fn get_path(user_config: &Configurations) -> String {
    let mut path = user_config.args.path.clone();

    if let None = path {
        let base_dirs = BaseDirs::new().unwrap();
        path = Some(base_dirs.data_dir().to_str().unwrap().to_string() + "\\pindow\\config.json");
    }
    path.unwrap()
}

fn add_to_config(user_config: &mut Configurations, path: String) {
    let app_commands = &mut user_config.configs[user_config.current_config].app_commands;
    app_commands.insert(app_commands.len(), config::options::AppCommand { app: path, args: vec![] })
}

fn add_to_file(path: String, process_path: String, user_config: &Configurations) {
    let str = config::load::load_string(path.clone());

    let mut data: config::options::ConfigurationsStr = serde_json::from_str(&str).unwrap();
    let mut configs = data.configs;
    let mut current_config = &mut configs[user_config.current_config];
    if let Some(apps) = &mut current_config.apps {
        apps.insert(apps.len(), config::options::AppCommandStr { app_path: process_path, args: None });

        current_config.apps = Some(apps.clone());
        configs[user_config.current_config] = current_config.clone();
        data.configs = configs;

        config::write::write_to_file(path, data);
    }
}

pub fn add_config(user_config: &mut Configurations) {
        if let Some(_) = utils::get_app_by_id(&user_config.get_current(), win::get_id(win::current_window()) as isize) {
            return;
        }
        if let Ok(process_path) = utils::get_current_path() {
            add_to_config(user_config, process_path.clone());

            let user_config = user_config.clone();
            let _ = thread::spawn(move || {
                let path = get_path(&user_config);

                add_to_file(path, process_path, &user_config);
            });
        }
}
