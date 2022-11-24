use crate::window::update;

#[derive(Clone)]
pub struct ApplicationWindows {
    pub window_id: Vec<i32>,
    pub pointer: usize,
    pub path: Option<String>
}

impl ApplicationWindows {
    pub fn new() -> Self {
        ApplicationWindows{
            window_id: vec![],
            pointer: 0,
            path: None
        }
    }

    pub fn cycle(&mut self) -> Option<i32> {
        if self.window_id.len() == 0 {
            return None;
        }

        self.pointer += 1;
        if self.pointer >= self.window_id.len().try_into().unwrap() {
            self.pointer = 0;
        }
        let window = self.window_id[self.pointer];
        Some(window)
    }

    pub fn current(&self) -> Option<i32> {
        if self.window_id.len() > 0 {
            Some(self.window_id[self.pointer])
        } else {
            None
        }
    }

    pub fn get_path(&self, command: String) -> Result<String, Box<dyn std::error::Error>> {
        unimplemented!("Have not implemented this yet!");
    }

    pub fn set_path(&mut self, path: String) {
        self.path = Some(path);
    }

    pub fn update(&self) {
        unimplemented!("Have not implemented this yet!");
    }
}
