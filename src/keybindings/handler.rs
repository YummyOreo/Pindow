use device_query::Keycode;
use std::time::SystemTime;

use crate::config::key;
use crate::config::options;

#[derive(Default, Clone, Debug)]
pub struct Handler {
    pub num: i8,
    pub num_time: Option<SystemTime>,
    pub current_keys: Vec<Keycode>,
    pub timeout: u128,
}

impl Handler {
    pub fn check_update(&self, keys: &Vec<Keycode>) -> bool {
        if keys.len() != self.current_keys.len() {
            return true;
        }
        for key_index in 0..keys.len() {
            if keys[key_index] != self.current_keys[key_index] {
                return true;
            }
        }
        false
    }

    pub fn check_num(&mut self) {
        let last_num = self.num;
        for key in self.current_keys.clone() {
            match key {
                Keycode::Key1 => {
                    self.num = 1;
                }
                Keycode::Key2 => {
                    self.num = 2;
                }
                Keycode::Key3 => {
                    self.num = 3;
                }
                Keycode::Key4 => {
                    self.num = 4;
                }
                Keycode::Key5 => {
                    self.num = 5;
                }
                Keycode::Key6 => {
                    self.num = 6;
                }
                Keycode::Key7 => {
                    self.num = 7;
                }
                Keycode::Key8 => {
                    self.num = 8;
                }
                Keycode::Key9 => {
                    self.num = 9;
                }
                Keycode::Escape | Keycode::Key0 => {
                    self.reset_num();
                }
                _ => {}
            }
        }

        if last_num != self.num {
            self.num_time = Some(SystemTime::now());
        }
    }

    pub fn check_num_time(&mut self) {
        if let Some(time) = self.num_time {
            if let Ok(time_esapsed) = time.elapsed() {
                if time_esapsed.as_millis() > self.timeout {
                    self.reset_num()
                }
            }
        }
    }

    pub fn reset_num(&mut self) {
        if let Some(_) = self.num_time {
            self.num_time = None;
            self.num = 0;
        }
    }

    pub fn check_keybind(&self, expected: &Vec<Keycode>) -> bool {
        if self.current_keys.len() != expected.len() {
            return false;
        }

        for key in self.current_keys.clone().into_iter() {
            if !expected.contains(&key) {
                return false;
            }
        }

        true
    }

    pub fn set_current_keys(&mut self, keys: Vec<Keycode>) {
        self.current_keys = keys;
    }

    pub fn check_keybinds(&self, user_config: &options::Options) -> Option<key::Event> {
        let keymaps = &user_config.get_current().keymaps;

        for keymap in keymaps {
            if self.check_keybind(&keymap.keys) {
                return Some(keymap.event.clone());
            }
        }

        None
    }
}
