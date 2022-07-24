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
            log!(verbose, "Cloning {}", r.name);
            info!("Cloning ({} mode): {}", config.base.mode, r.name);
            Command::new("git")
                .args(&["clone", &r.url, &r.name])
                // If a branch is specified, clone that specific branch
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
