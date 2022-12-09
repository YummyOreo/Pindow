use crate::config::options::Configurations;
use crate::keybindings::handler::Handler;
use crate::window::utils::win::popup;

pub fn change_config(user_configs: &mut Configurations, key_handler: &mut Handler) {
    let num = TryInto::<usize>::try_into(key_handler.num).unwrap();

    if num > 9 {
        return;
    }

    if num == 0 {
        user_configs.increment();
    } else {
        user_configs.set_current(num - 1);
    }
    key_handler.timeout = user_configs.get_current().timeout;
    println!("Current Config: {}", user_configs.get_current().name);
    popup(
        "Config Changed".to_string(),
        "Changed configuration to: ".to_string() + &user_configs.get_current().name,
    );
}
