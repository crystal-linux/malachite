use std::env;
use std::path::Path;
use std::process::Command;

use crate::create_config;

pub fn config() {
    // Generate new config file if not already present
    if !Path::exists("mlc.toml".as_ref()) {
        create_config();
    }

    // Open config file in user's editor of choice
    let editor = env::var("EDITOR").unwrap_or_else(|_| "nano".to_string());
    Command::new(editor)
        .arg("mlc.toml")
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}
