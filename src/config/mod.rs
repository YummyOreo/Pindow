use serde_json;

mod load;
pub mod options;

pub fn load() -> Result<options::Configurations, Box<dyn std::error::Error>> {
    let dir: String = String::from(".\\config.json");
    let str = load::load_string(dir);

    let data: options::ConfigurationsStr = serde_json::from_str(&str)?;
    let config = load::map_configurations(data.clone());

    Ok(config)
}
