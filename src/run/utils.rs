use regex::Regex;
use which::which;

use crate::config::options::Config;
use crate::window::utils::win;

pub fn get_app_by_id(user_config: &Config, id: isize) -> Option<usize> {
    if let Some(handle) = win::get_handle(id as u32) {
        if let Some(path) = win::get_process_file(handle) {
            for app in user_config.app_commands.clone() {
                if let Ok(app_path) = get_path(app.app.clone()) {
                    if app_path == path {
                        let command = app.app.clone();
                        // have to close it here
                        win::close_handle(handle);

                        let index = user_config
                            .app_commands
                            .iter()
                            .position(|r| r.app == command)
                            .unwrap();
                        return Some(index);
                    }
                }
            }
        }
        win::close_handle(handle);
    }
    None
}

#[derive(Debug)]
pub struct GetPathError {}
impl std::fmt::Display for GetPathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Could not get the path!")
    }
}

pub fn get_path(command: String) -> Result<String, GetPathError> {
    let re = Regex::new(r"[A-Za-z0-9]+:\\").unwrap();
    let path = which(command);
    match path {
        Ok(path) => {
            if let Some(path_str) = path.to_str() {
                Ok(re.replace_all(&path_str.to_string(), "").to_string())
            } else {
                Err(GetPathError {})
            }
        }
        Err(_) => Err(GetPathError {}),
    }
}
