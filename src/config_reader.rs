use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use serde::{Serialize, Deserialize};
use crate::server_config::ServerConfig;

pub fn read_config(path: &str) -> Result<HashMap<String, ServerConfig>, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let map = serde_json::from_reader(reader)?;
    Ok(map)
}