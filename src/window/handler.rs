use regex::Regex;
use which::which;

use crate::window::update;

// #[derive(Debug, Clone)]
// pub struct ApplicationWindows {
//     pub window_ids: Vec<i32>,
//     pub exclude: Vec<i32>,
//     pub pointer: usize,
//     pub path: Option<String>,
// }

// impl ApplicationWindows {
//     pub fn new() -> Self {
//         ApplicationWindows {
//             window_ids: vec![],
//             exclude: vec![],
//             pointer: 0,
//             path: None,
//         }
//     }

//     pub fn cycle(&mut self) -> Option<i32> {
//         if self.window_ids.len() == 0 {
//             return None;
//         }

//         self.pointer += 1;
//         if self.pointer >= self.window_ids.len().try_into().unwrap() {
//             self.pointer = 0;
//         }
//         let window = self.window_ids[self.pointer];
//         Some(window)
//     }

//     pub fn current(&self) -> Option<i32> {
//         if self.window_ids.len() > 0 {
//             Some(self.window_ids[self.pointer])
//         } else {
//             None
//         }
//     }

//     pub fn get_path(&mut self, command: String) -> Result<String, GetPathError> {
//         let re = Regex::new(r"[A-Za-z0-9]+:\\").unwrap();
//         let path = which(command);
//         match path {
//             Ok(path) => {
//                 if let Some(path_str) = path.to_str() {
//                     Ok(re.replace_all(&path_str.to_string(), "").to_string())
//                 } else {
//                     Err(GetPathError {})
//                 }
//             }
//             Err(_) => Err(GetPathError {}),
//         }
//     }

//     pub fn set_path(&mut self, path: String) {
//         self.path = Some(path);
//     }

//     pub fn update(&mut self, current_windows: &Vec<isize>) {
//         if let Some(path) = &self.path {
//             let updated_windows = update::get_windows_by_path(path.to_string(), current_windows);

//             let diff = update::get_diff(self.window_ids.clone(), updated_windows);

//             for window in diff.added {
//                 if self.exclude.contains(&window) {
//                     continue;
//                 }
//                 self.window_ids.insert(self.window_ids.len(), window);
//             }
//             for window in diff.removed {
//                 let index = self
//                     .window_ids
//                     .iter()
//                     .position(|&win| win == window)
//                     .unwrap();
//                 self.window_ids.remove(index);
//             }
//         }
//     }
// }

// #[derive(Debug)]
// pub struct GetPathError {}
// impl std::fmt::Display for GetPathError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "Could not get the path!")
//     }
// }
