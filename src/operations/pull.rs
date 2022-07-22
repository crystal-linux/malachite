use std::env;
use std::process::Command;

use crate::info;
use crate::{crash, internal::AppExitCode, log, workspace::read_cfg};

fn do_the_pulling(repos: Vec<String>, verbose: bool, smart_pull: bool) {
    for repo in repos {
        // Set root dir to return after each git pull
        let root_dir = env::current_dir().unwrap();
        log!(verbose, "Root dir: {:?}", root_dir);

        // Enter repo dir
        info!("Entering working directory: {}", &repo);
        env::set_current_dir(repo).unwrap();
        log!(verbose, "Current dir: {:?}", env::current_dir().unwrap());

        // Pull
        log!(verbose, "Pulling");
        if smart_pull {
            // Just update the remote
            log!(verbose, "Smart pull");
            Command::new("git")
                .args(&["remote", "update"])
                .spawn()
                .unwrap()
                .wait()
                .unwrap();

            // Check the repository status
            let output = Command::new("git").arg("status").output().unwrap();

            // If there are changes, pull normally
            if String::from_utf8(output.stdout)
                .unwrap()
                .to_string()
                .contains("Your branch is behind")
            {
                Command::new("git")
                    .arg("pull")
                    .spawn()
                    .unwrap()
                    .wait()
                    .unwrap();
            } else {
                // If there are no changes, alert the user
                info!("No changes to pull");
            }
        } else {
            // Pull normally
            log!(verbose, "Normal pull");
            Command::new("git")
                .arg("pull")
                .spawn()
                .unwrap()
                .wait()
                .unwrap();
        }
        // Return to root dir
        env::set_current_dir(root_dir).unwrap();
        log!(
            verbose,
            "Returned to root dir: {:?}",
            env::current_dir().unwrap()
        );
    }
}

pub fn pull(packages: Vec<String>, exclude: Vec<String>, verbose: bool) {
    // Read config file
    let config = read_cfg(verbose);
    log!(verbose, "Config: {:?}", config);
    // If no packages are specified, imply all
    let all = packages.is_empty();
    log!(verbose, "All: {}", all);
    // Read smart_pull from config
    let smart_pull = config.smart_pull;
    log!(verbose, "Smart pull: {}", smart_pull);

    // Read repos from config
    let repos = config
        .repo
        .iter()
        .map(|x| x.name.clone())
        .collect::<Vec<String>>();
    log!(verbose, "Repos: {:?}", repos);

    // Set repos_applicable for next function
    let mut repos_applicable = if all { repos } else { packages };
    log!(verbose, "Repos applicable: {:?}", repos_applicable);

    // Subtract exclude from repos_applicable
    if !exclude.is_empty() {
        for ex in &exclude {
            repos_applicable.retain(|x| *x != *ex);
        }
    }
    log!(verbose, "Exclude: {:?}", exclude);
    log!(verbose, "Repos applicable excluded: {:?}", repos_applicable);

    // If all is not specified and packages is empty, crash
    if repos_applicable.is_empty() {
        crash!(AppExitCode::NoPkgs, "No packages specified");
    }

    // Pull!
    log!(verbose, "Pulling {:?}", repos_applicable);
    do_the_pulling(repos_applicable, verbose, smart_pull);
}
