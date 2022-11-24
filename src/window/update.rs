pub struct WindowsDiff {
    added: Option<Vec<isize>>,
    same: Option<Vec<isize>>,
    removed: Option<Vec<isize>>,
}

pub fn get_diff(windows_1: Vec<isize>, windows_2: Vec<isize>) -> Result<WindowsDiff, Box<dyn std::error::Error>> {
    unimplemented!("Have not implemented this yet!");
}

pub fn get_windows() {}
