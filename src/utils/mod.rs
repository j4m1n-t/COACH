pub mod logging;
pub mod permission;
use std::fs;
use std::path::Path;


pub fn load_targets<P: AsRef<Path>>(file: P) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file)?;
    let targets: Vec<String> = content
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .collect();

    if targets.is_empty() {
        Err("No valid targets found in the file".into())
    } else {
        Ok(targets)
    }
}