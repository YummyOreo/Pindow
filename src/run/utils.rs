use which::which;

use crate::config::options::Config;
use crate::window::utils::win;

pub fn get_app_by_id(user_config: &Config, id: isize) -> Option<usize> {
    if let Some(handle) = win::get_handle(id as u32) {
        if let Some(path) = win::get_process_file(handle) {
            for app in &user_config.app_commands {
                if let Ok(app_path) = get_path(&app.app) {
                    if app_path == path {
                        let command = &app.app;
                        // have to close it here
                        win::close_handle(handle);

                        let index = user_config
                            .app_commands
                            .iter()
                            .position(|r| &r.app == command)
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

pub fn get_path(command: &String) -> Result<String, GetPathError> {
    let path = which(command);
    match path {
        Ok(path) => {
            if let Some(path_str) = path.to_str() {
                Ok(path_str.to_string())
            } else {
                Err(GetPathError {})
            }
        }
        Err(_) => Err(GetPathError {}),
    }
}

pub fn get_current_path() -> Result<String, GetPathError> {
    let current_window = win::get_id(win::current_window());
    if let Some(handle) = win::get_handle(current_window as u32) {
        if let Some(path) = win::get_process_file(handle) {
            win::close_handle(handle);
            return Ok(path);
        }
        win::close_handle(handle);
    }
    Err(GetPathError {})
}
