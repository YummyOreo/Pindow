use crate::config::options::Options;
use crate::keybindings::handler::Handler;
use crate::window::utils::win::popup;

fn notify_user(key_handler: &mut Handler, user_configs: &mut Options) -> Option<()> {
    key_handler.timeout = user_configs.get_current().timeout;
    println!("Current Config: {}", user_configs.get_current().name);
    popup(
        "Config Changed".to_string(),
        "Changed configuration to: ".to_string() + &user_configs.get_current().name,
    );
    Some(())
}

pub fn set_config(user_configs: &mut Options, key_handler: &mut Handler, num: usize) -> Option<()> {
    if num >= user_configs.configs.len() {
        return None;
    }

    match user_configs.set_current(num) {
        Ok(_) => notify_user(key_handler, user_configs),
        Err(e) => {
            println!("{}", e);
            None
        }
    }
}

pub fn incement_config(user_configs: &mut Options, key_handler: &mut Handler) -> Option<()> {
    user_configs.increment();

    notify_user(key_handler, user_configs)
}

pub fn decrement_config(user_configs: &mut Options, key_handler: &mut Handler) -> Option<()> {
    user_configs.decrement_config();

    notify_user(key_handler, user_configs)
}

pub fn inc_set_config(user_configs: &mut Options, key_handler: &mut Handler) -> Option<()> {
    let num = TryInto::<usize>::try_into(key_handler.num).unwrap();

    if num > 9 {
        return None;
    }

    if num == 0 {
        incement_config(user_configs, key_handler)
    } else {
        set_config(user_configs, key_handler, num - 1)
    }
}

pub fn dec_set_config(user_configs: &mut Options, key_handler: &mut Handler) -> Option<()> {
    let num = TryInto::<usize>::try_into(key_handler.num).unwrap();

    if num > 9 {
        return None;
    }

    if num == 0 {
        decrement_config(user_configs, key_handler)
    } else {
        set_config(user_configs, key_handler, num - 1)
    }
}
