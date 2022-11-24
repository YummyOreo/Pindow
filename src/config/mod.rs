use serde_json;

mod load;
pub mod options;

pub fn load() -> Result<options::Config, Box<dyn std::error::Error>> {
    let dir: String = String::from(".\\config.json");
    let str = load::load_string(dir);

    let data: options::ConfigStr = serde_json::from_str(&str)?;
    let config = load::map_config(data.clone());

    Ok(config)
}
