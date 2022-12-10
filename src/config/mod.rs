use serde_json;
use directories::{BaseDirs, UserDirs, ProjectDirs};

mod load;
pub mod options;

pub fn load() -> Option<options::Configurations> {
    // let dir: String = String::from(".\\config.json");
    //
    if let Some(base_dirs) = BaseDirs::new() {
        let path = base_dirs.data_dir().to_str().unwrap().to_string() + "\\pindow\\config.json";
        let str = load::load_string(path);

        let data: options::ConfigurationsStr = serde_json::from_str(&str).unwrap();
        let config = load::map_configurations(data.clone());

        return Some(config);
    }
    None
}
