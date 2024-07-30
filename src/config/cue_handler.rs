use std::fs;
use std::path::Path;
use crate::config::{ConfigData, Task, GlobalSettings};

pub fn load_cue<P: AsRef<Path>>(path: P) -> Result<ConfigData, Box<dyn std::error::Error>> {
    // cue-lang is working on a rust api. Once this is finished we can implement
    
    let content = fs::read_to_string(path)?;
    unimplemented!("CUE parsing not implemented")
}