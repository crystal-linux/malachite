use std::env;
use std::process::Command;

use crate::{info, log};

pub fn clone(verbose: bool) {
    // Read config struct from mlc.toml
    let config = crate::internal::parse_cfg(verbose);
    log!(verbose, "Config: {:?}", config);
    let repos = &config.repositories;
    log!(verbose, "Repos: {:?}", repos);

    // Get a vector of all files/dirs in the current directory, excluding config file
    let dir_paths = std::fs::read_dir("./").unwrap();
    let mut dirs = dir_paths
        .map(|x| x.unwrap().path().display().to_string())
        .collect::<Vec<String>>();
    dirs.retain(|x| *x != "./mlc.toml");
    dirs.retain(|x| *x != "./out");
    if config.mode.repository.is_some() {
        dirs.retain(|x| *x != format!("./{}", config.mode.repository.as_ref().unwrap().name));
    }
    log!(verbose, "Paths with mlc.toml excluded: {:?}", dirs);

    // Creates a vector of the difference between cloned repos and repos defined in config
    let mut repo_diff = vec![];
    for repo in repos {
        let name = &repo.name;
        if !dirs.contains(&format!("./{}", name)) {
            repo_diff.push(repo);
        }
    }

    // Diff logic
    if repo_diff.is_empty() {
        // No diff, do nothing
        log!(verbose, "No diff");
        info!("All repos are already cloned");
    } else {
        log!(verbose, "Diff: {:?}", repo_diff);
        // This is just for pretty display purposes
        let display = repo_diff
            .iter()
            .map(|x| x.name.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        info!("New/missing repos to clone: {}", display);

        // Clone all diff repos
        for r in repo_diff {
            log!(verbose, "Depth: {:?}", r.extra);
            log!(verbose, "Cloning {}", r.name);
            if r.extra.is_some() && config.base.mode == "workspace" {
                info!(
                    "Cloning ({} mode): {} at depth: {}",
                    config.base.mode,
                    r.name,
                    r.extra.as_ref().unwrap()
                );
            } else if r.extra.is_some() && config.base.mode == "repository" {
                info!(
                    "Cloning ({} mode): {} at {}",
                    config.base.mode,
                    r.name,
                    r.extra.as_ref().unwrap()
                );
            } else {
                info!("Cloning ({} mode): {}", config.base.mode, r.name);
            }

            if r.extra.is_some() && config.base.mode == "workspace" {
                // Clone with specified extra depth
                Command::new("git")
                    .args(&["clone", &r.url, &r.name])
                    // If a branch is specified, clone that specific branch
                    .args(if r.branch.is_some() {
                        vec!["-b", r.branch.as_ref().unwrap()]
                    } else {
                        vec![]
                    })
                    .args(if r.extra.is_some() {
                        vec!["--depth", r.extra.as_ref().unwrap()]
                    } else {
                        vec![]
                    })
                    .spawn()
                    .unwrap()
                    .wait()
                    .unwrap();
            } else if config.base.mode == "repository" {
                // Clone and checkout specified hash
                // Create an empty directory with repo.name and enter it
                let root_dir = env::current_dir().unwrap();

                // Git clone the repo with the `-n` flag to not immediately checkout the files
                Command::new("git")
                    .args(&["clone", &r.url, &r.name, "-n"])
                    .args(if r.branch.is_some() {
                        vec!["-b", r.branch.as_ref().unwrap()]
                    } else {
                        vec![]
                    })
                    .spawn()
                    .unwrap()
                    .wait()
                    .unwrap();

                std::env::set_current_dir(&r.name).unwrap();
                log!(verbose, "Entered directory: {}", r.name);

                // Git checkout the PKGBUILD from the hash
                if r.extra.is_some() {
                    Command::new("git")
                        .args(&["checkout", r.extra.as_ref().unwrap(), "PKGBUILD"])
                        .spawn()
                        .unwrap()
                        .wait()
                        .unwrap();
                } else {
                    Command::new("git")
                        .args(&["checkout", "HEAD", "PKGBUILD"])
                        .spawn()
                        .unwrap()
                        .wait()
                        .unwrap();
                }

                // Return to the root directory
                std::env::set_current_dir(root_dir).unwrap();
                log!(verbose, "Returned to root directory");
            } else {
                // Clone normally
                Command::new("git")
                    .args(&["clone", &r.url, &r.name])
                    .args(if r.branch.is_some() {
                        vec!["-b", r.branch.as_ref().unwrap()]
                    } else {
                        vec![]
                    })
                    .spawn()
                    .unwrap()
                    .wait()
                    .unwrap();
            }
        }
    }
}
