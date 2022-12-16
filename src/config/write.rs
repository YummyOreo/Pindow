use serde_json;

use crate::config::options::OptionsStr;

pub fn write_to_file(path: String, config: OptionsStr) {
    let str = serde_json::to_string(&config).unwrap();
    std::fs::write(path, str).expect("Unable to write file");
}
