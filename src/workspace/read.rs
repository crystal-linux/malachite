use std::fs;
use std::path::Path;

use crate::internal::structs::Config;

pub fn read_cfg() -> Config {
    if !Path::exists("mlc.toml".as_ref()) {
        panic!("mlc.toml file not found")
    }

    let file = fs::read_to_string("mlc.toml").unwrap();
    let config: Config = toml::from_str(&file).unwrap();

    config
}