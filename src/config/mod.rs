use serde_json;
use directories::BaseDirs;

mod load;
pub mod options;

pub fn load(path: Option<String>) -> options::Configurations {
    let mut path = path;

    if let None = path {
        let base_dirs = BaseDirs::new().unwrap();
        path = Some(base_dirs.data_dir().to_str().unwrap().to_string() + "\\pindow\\config.json");
    }
    let str = load::load_string(path.unwrap());

    let data: options::ConfigurationsStr = serde_json::from_str(&str).unwrap();
    let config = load::map_configurations(data.clone());

    config
}
