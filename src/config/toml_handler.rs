use std::fs;
use std::path::Path;
use toml;
use crate::config::{ConfigData, Task, GlobalSettings};

pub fn load_toml<P: AsRef<Path>>(path: P) -> Result<ConfigData, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let parsed: toml::Value = toml::from_str(&content)?;
    
    let tasks = parsed["tasks"].as_array()
        .ok_or("Missing tasks array")?
        .iter()
        .map(|task| {
            Ok(Task {
                name: task["name"].as_str().ok_or("Missing task name")?.to_string(),
                action: task["action"].as_str().ok_or("Missing task action")?.to_string(),
                params: task["params"].as_table()
                    .ok_or("Missing task params")?
                    .clone()
                    .try_into()?,
            })
        })
        .collect::<Result<Vec<Task>, Box<dyn std::error::Error>>>()?;

    let global_settings = GlobalSettings {
        default_protocol: parsed["global_settings"]["default_protocol"]
            .as_str()
            .unwrap_or("tcp")
            .to_string(),
        timeout: parsed["global_settings"]["timeout"]
            .as_integer()
            .unwrap_or(30) as u64,
    };

    Ok(ConfigData {
        tasks,
        global_settings,
    })
}