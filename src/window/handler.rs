use regex::Regex;

use crate::window::update;

#[derive(Debug, Clone)]
pub struct ApplicationWindows {
    pub window_ids: Vec<i32>,
    pub pointer: usize,
    pub path: Option<String>,
}

impl ApplicationWindows {
    pub fn new() -> Self {
        ApplicationWindows {
            window_ids: vec![],
            pointer: 0,
            path: None,
        }
    }

    pub fn cycle(&mut self) -> Option<i32> {
        if self.window_ids.len() == 0 {
            return None;
        }

        self.pointer += 1;
        if self.pointer >= self.window_ids.len().try_into().unwrap() {
            self.pointer = 0;
        }
        let window = self.window_ids[self.pointer];
        Some(window)
    }

    pub fn current(&self) -> Option<i32> {
        if self.window_ids.len() > 0 {
            Some(self.window_ids[self.pointer])
        } else {
            None
        }
    }

    pub fn get_path(&self, command: String) -> Result<String, Box<dyn std::error::Error>> {
        let re = Regex::new(r"[A-Za-z0-9]+:\\");
        unimplemented!("Have not implemented this yet!");
    }

    pub fn set_path(&mut self, path: String) {
        self.path = Some(path);
    }

    pub fn update(&mut self, current_windows: &Vec<isize>) {
        if let Some(path) = &self.path {
            let updated_windows = update::get_windows_by_path(path.to_string(), current_windows);

            let diff = update::get_diff(self.window_ids.clone(), updated_windows);

            for window in diff.added {
                self.window_ids.insert(self.window_ids.len(), window);
            }
            for window in diff.removed {
                let index = self.window_ids.iter().position(|&win| win == window).unwrap();
                self.window_ids.remove(index);
            }
        }
    }
}
