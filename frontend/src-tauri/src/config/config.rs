use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub api_host: String,
}

lazy_static! {
    pub static ref CONFIG: Config = {
        let config = std::fs::read_to_string("src/config.yaml").unwrap();
        serde_yaml::from_str(&config).unwrap()
    };
}

pub fn get_config() -> Config {
    CONFIG.clone()
}