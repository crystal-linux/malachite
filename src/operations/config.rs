use std::env;
use std::path::Path;
use std::process::Command;

use crate::{log, repository::create};

pub fn config(verbose: bool) {
    // Generate new config file if not already present
    if !Path::exists("mlc.toml".as_ref()) {
        log!(verbose, "Creating mlc.toml");
        create(verbose);
    }

    // Open config file in user's editor of choice
    let editor = env::var("EDITOR").unwrap_or_else(|_| "nano".to_string());
    log!(verbose, "Opening mlc.toml in {}", editor);
    Command::new(editor)
        .arg("mlc.toml")
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}
