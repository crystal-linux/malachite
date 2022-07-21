use std::path::Path;
use std::process::Command;
use std::{env, fs};

use crate::workspace::read_cfg;

pub fn generate() {
    // Read config struct from mlc.toml
    let config = read_cfg();

    // Get repository name from config
    let name = config.name.unwrap();

    // If repository exists, delete it
    if Path::exists(name.as_ref()) {
        fs::remove_dir_all(&name).unwrap();
    }

    // Create or recreate repository directory
    fs::create_dir_all(&name).unwrap();

    // Copy out packages to repository directory
    Command::new("bash")
        .args(&["-c", &format!("cp -v out/* {}/", &name)])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    // Enter repository directory
    env::set_current_dir(&name).unwrap();

    let db = format!("{}.db", &name);
    let files = format!("{}.files", &name);

    // Create repo.db and repo.files using repo-add
    Command::new("bash")
        .args(&[
            "-c",
            &format!("GLOBIGNORE=\"*.sig\" repo-add {}.tar.gz *.pkg.tar.*", db),
        ])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    // Replace repo.{db,files}.tar.gz with just repo.{db,files}
    Command::new("bash")
        .args(&[
            "-c",
            &format!("mv {}.tar.gz {}; mv {}.tar.gz {}", db, db, files, files),
        ])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}
