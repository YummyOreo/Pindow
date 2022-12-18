use crate::config::options::Options;
use crate::keybindings::handler::Handler;
use crate::window::utils::win::popup;

pub fn set_config(user_configs: &mut Options, key_handler: &mut Handler, num: usize) {
    if num >= user_configs.configs.len() {
        return;
    }

    let _ = user_configs.set_current(num);

    key_handler.timeout = user_configs.get_current().timeout;
    println!("Current Config: {}", user_configs.get_current().name);
    popup(
        "Config Changed".to_string(),
        "Changed configuration to: ".to_string() + &user_configs.get_current().name,
    );
}

pub fn incement_config(user_configs: &mut Options, key_handler: &mut Handler) {
    user_configs.increment();

    key_handler.timeout = user_configs.get_current().timeout;
    println!("Current Config: {}", user_configs.get_current().name);
    popup(
        "Config Changed".to_string(),
        "Changed configuration to: ".to_string() + &user_configs.get_current().name,
    );
}

pub fn decrement_config(user_configs: &mut Options, key_handler: &mut Handler) {
    user_configs.decrement_config();

    key_handler.timeout = user_configs.get_current().timeout;
    println!("Current Config: {}", user_configs.get_current().name);
    popup(
        "Config Changed".to_string(),
        "Changed configuration to: ".to_string() + &user_configs.get_current().name,
    );
}

pub fn inc_set_config(user_configs: &mut Options, key_handler: &mut Handler) {
    let num = TryInto::<usize>::try_into(key_handler.num).unwrap();

    if num > 9 {
        return;
    }

    if num == 0 {
        incement_config(user_configs, key_handler);
    } else {
        set_config(user_configs, key_handler, num - 1);
    }
}

pub fn dec_set_config(user_configs: &mut Options, key_handler: &mut Handler) {
    let num = TryInto::<usize>::try_into(key_handler.num).unwrap();

    if num > 9 {
        return;
    }

    if num == 0 {
        decrement_config(user_configs, key_handler);
    } else {
        set_config(user_configs, key_handler, num - 1);
    }
}
