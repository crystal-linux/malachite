use std::path::Path;
use std::process::Command;
use std::{env, fs};

use crate::{workspace::read_cfg, log};

pub fn generate() {
    // Read config struct from mlc.toml
    let config = read_cfg();
    log!("Config: {:?}", config);

    // Get repository name from config
    let name = config.name.unwrap();
    log!("Name: {}", name);

    // If repository exists, delete it
    if Path::exists(name.as_ref()) {
        log!("Deleting {}", name);
        fs::remove_dir_all(&name).unwrap();
    }

    // Create or recreate repository directory
    fs::create_dir_all(&name).unwrap();
    log!("Created {}", name);

    // Copy out packages to repository directory
    Command::new("bash")
        .args(&["-c", &format!("cp -v out/* {}/", &name)])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
    log!("Copied out packages to {}", name);

    // Enter repository directory
    env::set_current_dir(&name).unwrap();
    log!("Current dir: {:?}", env::current_dir().unwrap());

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
    log!("Created {} and {}", db, files);

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
    log!("Renamed {}.tar.gz to {} and {}.tar.gz to {}", db, db, files, files);
}
