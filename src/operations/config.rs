use std::env;
use std::path::Path;
use std::process::Command;

use crate::create_config;

pub fn config() {
    if !Path::exists("mlc.toml".as_ref()) {
        create_config();
    }
    let editor = env::var("EDITOR").unwrap_or_else(|_| "nano".to_string());
    Command::new(editor)
        .arg("mlc.toml")
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}
