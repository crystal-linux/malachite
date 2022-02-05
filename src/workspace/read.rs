use crate::crash;
use std::fs;
use std::path::Path;

use crate::internal::structs::{Config, SplitRepo, UnexpandedConfig};

pub fn read_cfg() -> Config {
    if !Path::exists("mlc.toml".as_ref()) {
        crash("Config file not found (mlc.toml)".to_string(), 5)
    }

    let file = fs::read_to_string("mlc.toml").unwrap();
    let config: UnexpandedConfig = toml::from_str(&file).unwrap();

    let mut trimmed_urls: Vec<String> = vec![];
    let mut expanded_repos: Vec<String> = vec![];

    for url in config.urls {
        let a = url.split("%repo%").collect::<Vec<&str>>()[0];
        let mut b = vec![a.to_string()];
        trimmed_urls.append(&mut b);
    }

    let config_repos = config.repo;
    for x in config_repos {
        let split: Vec<&str> = x.split("::").collect();
        let sr_struct = SplitRepo {
            indx: split[0].parse().unwrap(),
            name: split[1].parse().unwrap(),
        };
        let index = sr_struct.indx;
        let expanded = format!("{}{}", trimmed_urls[index - 1], sr_struct.name);
        expanded_repos.push(expanded);
    }

    Config {
        mode: config.mode,
        name: config.name,
        repo: expanded_repos,
    }
}
