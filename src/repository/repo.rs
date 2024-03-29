use std::path::Path;
use std::process::Command;
use std::{env, fs};

use crate::{crash, info, internal::parse_cfg, internal::AppExitCode, log};

pub fn generate(verbose: bool) {
    // Read config struct from mlc.toml
    let config = parse_cfg(verbose);
    log!(verbose, "Config: {:?}", config);

    // Get signing from config
    let signing = &config.mode.repository.as_ref().unwrap().signing.enabled;
    log!(verbose, "Signing: {:?}", signing);

    // Get repository name from config
    let name = &config.mode.repository.as_ref().unwrap().name;
    log!(verbose, "Name: {}", name);

    // Read on_gen from config
    let on_gen = &config.mode.repository.as_ref().unwrap().signing.on_gen;
    log!(verbose, "On gen: {:?}", on_gen);

    // Read key from config
    let key = &config.mode.repository.as_ref().unwrap().signing.key;
    log!(verbose, "Key: {:?}", key);

    info!("Generating repository: {}", name);

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

    // Enter repository directory
    env::set_current_dir(&name).unwrap();
    log!(verbose, "Current dir: {:?}", env::current_dir().unwrap());

    // Sign all package files in repository if signing and on_gen are true
    if *signing && on_gen.is_some() && on_gen.unwrap() {
        // Get a list of all .tar.* files in repository
        let files = fs::read_dir(".").unwrap();

        for file in files {
            // Get file name
            let file = file.unwrap();
            let path = file.path();

            let sign_command = if key.is_some() && !key.as_ref().unwrap().is_empty() {
                format!(
                    "gpg --default-key {} --detach-sign {}",
                    key.as_ref().unwrap(),
                    path.to_str().unwrap()
                )
            } else {
                format!("gpg --detach-sign {}", path.to_str().unwrap())
            };

            // If extension is either .zst or .xz, sign it
            if path.extension().unwrap() == "zst" || path.extension().unwrap() == "xz" {
                log!(verbose, "Signing {}", path.display());
                Command::new("bash")
                    .arg("-c")
                    .args(&[&sign_command])
                    .spawn()
                    .unwrap()
                    .wait()
                    .unwrap();
            }
        }
        log!(verbose, "Signed repository");
    }

    let db = format!("{}.db", &name);
    let files = format!("{}.files", &name);

    // Check if package files end with .tar.zst or .tar.xz
    let zst = Command::new("bash")
        .args(&["-c", "ls *.tar.zst"])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
    let xz = Command::new("bash")
        .args(&["-c", "ls *.tar.xz"])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    // This should never happen, crash and burn if it does
    if zst.success() && xz.success() {
        crash!(
            AppExitCode::RepoParseError,
            "Both .tar.zst and .tar.xz files found in repository. You've done something wrong. Aborting"
        );
    }

    // Ensuring aarch64/ALARM support for the future
    let aarch64_mode = if zst.success() {
        false
    } else if xz.success() {
        true
    } else {
        crash!(
            AppExitCode::PkgsNotFound,
            "No .zst or .xz packages found in repository"
        );
        // This should theoretically never be reached, but let's just give the compiler what it wants
        false
    };
    let suffix = if aarch64_mode { "xz" } else { "zst" };

    // Create repo.db and repo.files using repo-add
    Command::new("bash")
        .args(&[
            "-c",
            &format!(
                "GLOBIGNORE=\"*.sig\" repo-add {}.tar.gz *.pkg.tar.{}",
                db, suffix
            ),
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
