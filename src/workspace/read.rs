use std::fs;
use std::path::Path;
use crate::crash;

use crate::internal::structs::Config;

pub fn read_cfg() -> Config {
    if !Path::exists("mlc.toml".as_ref()) {
        crash("Config file not found (mlc.toml)".to_string(), 5)
    }

    let file = fs::read_to_string("mlc.toml").unwrap();
    let config: Config = toml::from_str(&file).unwrap();

    config
}