use std::process::Command;
use std::thread;

use crate::config::options::Config;
use crate::keybindings::handler::Handler;

pub fn run_app(user_config: &Config, key_handler: &Handler) {
    let num = TryInto::<usize>::try_into(key_handler.num).unwrap();

    if num == 0 || num > 9 {
        return;
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
