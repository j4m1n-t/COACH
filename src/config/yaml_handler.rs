use std::collections::HashMap;
use std::fs;
use std::path::Path;
use serde_yaml::{self, Value};
use crate::config::{ConfigData, Task, GlobalSettings};

pub fn load_yaml<P: AsRef<Path>>(path: P) -> Result<ConfigData, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let parsed: Value = serde_yaml::from_str(&content)?;

    let tasks = parsed["tasks"].as_sequence()
        .ok_or("Missing tasks array")?
        .iter()
        .map(|task| {
            let params_map = match task["params"].as_mapping() {
                Some(mapping) => mapping.iter().map(|(k, v)| {
                    let key = k.as_str().ok_or("Non-string key in params")?.to_string();
                    let value = serde_json::to_value(v).map_err(|_| "Failed to convert value to JSON")?;
                    Ok((key, value))
                }).collect::<Result<HashMap<String, serde_json::Value>, Box<dyn std::error::Error>>>()?,
                None => HashMap::new(),
            };

            Ok(Task {
                name: task["name"].as_str().ok_or("Missing task name")?.to_string(),
                action: task["action"].as_str().ok_or("Missing task action")?.to_string(),
                params: params_map,
            })
        })
        .collect::<Result<Vec<Task>, Box<dyn std::error::Error>>>()?;

    let global_settings = GlobalSettings {
        default_protocol: parsed["global_settings"]["default_protocol"]
            .as_str()
            .unwrap_or("tcp")
            .to_string(),
        timeout: parsed["global_settings"]["timeout"]
            .as_i64()
            .unwrap_or(30) as u64,
    };

    Ok(ConfigData {
        tasks,
        global_settings,
    })
}