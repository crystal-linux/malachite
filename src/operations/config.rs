use std::env;
use std::path::Path;
use std::process::Command;

use crate::{create_config, log};

pub fn config() {
    // Generate new config file if not already present
    if !Path::exists("mlc.toml".as_ref()) {
        log!("Creating mlc.toml");
        create_config();
    }

    // Open config file in user's editor of choice
    let editor = env::var("EDITOR").unwrap_or_else(|_| "nano".to_string());
    log!("Opening mlc.toml in {}", editor);
    Command::new(editor)
        .arg("mlc.toml")
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}
