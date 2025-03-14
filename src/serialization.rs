use serde::{Serialize, Deserialize};
use serde_json;
use toml;
use serde_yaml;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub name: String,
    pub version: String,
}

pub fn json_serialize(config: &Config) -> String {
    serde_json::to_string(config).unwrap()
}

pub fn json_deserialize(data: &str) -> Config {
    serde_json::from_str(data).unwrap()
}
