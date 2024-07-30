// src/config/mod.rs
pub mod toml_handler;
pub mod cue_handler;
pub mod yaml_handler;
use std::path::Path;

pub fn load_config<P: AsRef<Path>>(path: P) -> Result<ConfigData, Box<dyn std::error::Error>> {
    if path.as_ref().extension().unwrap_or_default() == "toml" {
        toml_handler::load_toml(path)
    } else if path.as_ref().extension().unwrap_or_default() == "cue" {
        cue_handler::load_cue(path)
    } else if path.as_ref().extension().unwrap_or_default() == "yaml" {
        yaml_handler::load_yaml(path) }
        else {
        Err("Unsupported file format".into())
    }
}

pub struct ConfigData {
    pub tasks: Vec<Task>,
    pub global_settings: GlobalSettings,
}

pub struct Task {
    pub name: String,
    pub action: String,
    pub params: std::collections::HashMap<String, serde_json::Value>,
}

pub struct GlobalSettings {
    pub default_protocol: String,
    pub timeout: u64,
    // Add other global settings as needed
}