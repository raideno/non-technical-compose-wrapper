use std::fs;
use serde_json;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Assistance {
    pub enabled: bool,
    pub email: String,
    pub phone: String,
}

#[derive(Debug, Deserialize)]
pub struct Configuration {
    pub assistance: Assistance,
}

pub fn load(path: String) -> Result<Configuration, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let configuration: Configuration = serde_json::from_str(&content)?;
    Ok(configuration)
}
