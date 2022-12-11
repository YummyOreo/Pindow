use serde_json;

use crate::config::options::ConfigurationsStr;

pub fn write_to_file(path: String, config: ConfigurationsStr) {
    let str = serde_json::to_string(&config).unwrap();
    std::fs::write(path, str).expect("Unable to write file");
}
