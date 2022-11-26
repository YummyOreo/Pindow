use crate::window::utils;

pub struct WindowsDiff {
    pub added: Vec<i32>,
    pub same: Vec<i32>,
    pub removed: Vec<i32>,
}

pub fn get_diff(current: Vec<i32>, updated: Vec<i32>) -> WindowsDiff {
    let mut diff: WindowsDiff = WindowsDiff {
        added: vec![],
        same: vec![],
        removed: vec![],
    };
    for window in &updated {
        if current.contains(&window) {
            diff.same.insert(0, *window);
        } else {
            diff.added.insert(0, *window);
        }
    }

    for window in current {
        if !(updated.contains(&window)) {
            diff.removed.insert(0, window);
        }
    }
    diff
}

pub fn get_windows_by_path(path: String, current_windows: &Vec<isize>) -> Vec<i32> {
    let mut windows: Vec<i32> = vec![];
    for window in current_windows {
        let id = utils::win::get_id(*window);
        if let Some(handle) = utils::win::get_handle(id) {
            if let Some(window_path) = utils::win::get_process_file(handle) {
                if str::to_lowercase(&window_path) == str::to_lowercase(&path) {
                    windows.insert(0, window.clone() as i32);
                };
            };

            utils::win::close_handle(handle);
        };
    }
    windows
}
