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

pub fn add_config(user_config: &Configurations) {
    if let None = utils::get_app_by_id(&user_config.get_current(), win::get_id(win::current_window()) as isize) {
        if let Ok(process_path) = utils::get_current_path() {
            let mut path = user_config.args.path.clone();

            if let None = path {
                let base_dirs = BaseDirs::new().unwrap();
                path = Some(base_dirs.data_dir().to_str().unwrap().to_string() + "\\pindow\\config.json");
            }
            let str = config::load::load_string(path.clone().unwrap());

            let mut data: config::options::ConfigurationsStr = serde_json::from_str(&str).unwrap();
            let mut configs = data.configs;
            let mut current_config = &mut configs[user_config.current_config];
            if let Some(apps) = &mut current_config.apps {
                apps.insert(apps.len(), config::options::AppCommandStr { app_path: process_path, args: None });

                current_config.apps = Some(apps.clone());
                configs[user_config.current_config] = current_config.clone();
                data.configs = configs;

                let str = serde_json::to_string(&data).unwrap();
                std::fs::write(path.unwrap(), str).expect("Unable to write file");
            }
        }
    }
}
