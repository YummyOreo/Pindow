use directories::BaseDirs;

pub mod key;
pub mod load;
pub mod options;
pub mod write;

pub fn load(path: &Option<String>) -> options::Options {
    let str = match path {
        None => {
            let base_dirs = BaseDirs::new().unwrap();
            let base_path =
                base_dirs.data_dir().to_str().unwrap().to_string() + "\\pindow\\config.json";
            load::load_string(&base_path)
        }
        Some(path) => load::load_string(path),
    };

    let data: options::OptionsStr = serde_json::from_str(&str).unwrap();
    load::map_options(data)
}
