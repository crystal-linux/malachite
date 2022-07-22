use std::path::Path;
use std::process::Command;
use std::{env, fs};

use crate::{log, workspace::read_cfg};

pub fn generate(verbose: bool) {
    // Read config struct from mlc.toml
    let config = read_cfg(verbose);
    log!(verbose, "Config: {:?}", config);

    // Get repository name from config
    let name = config.mode.repository.name;
    log!(verbose, "Name: {}", name);

    // If repository exists, delete it
    if Path::exists(name.as_ref()) {
        log!(verbose, "Deleting {}", name);
        fs::remove_dir_all(&name).unwrap();
    }

    // Create or recreate repository directory
    fs::create_dir_all(&name).unwrap();
    log!(verbose, "Created {}", name);

    // Copy out packages to repository directory
    Command::new("bash")
        .args(&["-c", &format!("cp -v out/* {}/", &name)])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
    log!(verbose, "Copied out packages to {}", name);

    // Sign all package files in repository if signing and on_gen are true
    if config.mode.repository.signing.enabled && config.mode.repository.signing.on_gen {
        // Directory stuff
        let dir = env::current_dir().unwrap();
        log!(verbose, "Current dir: {:?}", dir);
        env::set_current_dir(&name).unwrap();
        log!(verbose, "Current dir: {:?}", env::current_dir().unwrap());
        // Get a list of all .tar.* files in repository
        let files = fs::read_dir("./").unwrap();
        for file in files {
            let file = file.unwrap();
            let path = file.path();
            if path.extension().unwrap() == "zst" || path.extension().unwrap() == "xz" {
                log!(verbose, "Signing {}", path.display());
                Command::new("bash")
                    .args(&["-c", &format!("gpg --default-key {} --detach-sign {}", config.mode.repository.signing.key, file.file_name().to_str().unwrap())])
                    .spawn()
                    .unwrap()
                    .wait()
                    .unwrap();
            }
        }
        // Return to root dir
        env::set_current_dir(dir).unwrap();
        log!(verbose, "Current dir: {:?}", env::current_dir().unwrap());
        log!(verbose, "Signed repository");
    }

    // Enter repository directory
    env::set_current_dir(&name).unwrap();
    log!(verbose, "Current dir: {:?}", env::current_dir().unwrap());

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
    log!(verbose, "Created {} and {}", db, files);

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
    log!(
        verbose,
        "Renamed {}.tar.gz to {} and {}.tar.gz to {}",
        db,
        db,
        files,
        files
    );
}
