use std::{collections::HashMap, fs, path};

pub fn read_config() -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let config_string = fs::read_to_string(
        path::Path::new(&std::env::current_dir().unwrap()).join("bapao.config.json"),
    )?;

    let config: HashMap<String, String> = serde_json::from_str(&config_string)?;

    return Result::Ok(config);
}
