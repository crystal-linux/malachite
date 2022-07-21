use std::env;
use std::process::Command;

use crate::info;
use crate::{crash, internal::AppExitCode, log};

fn do_the_pulling(repos: Vec<String>, verbose: bool) {
    for repo in repos {
        // Set root dir to return after each git pull
        let root_dir = env::current_dir().unwrap();
        log!(verbose, "Root dir: {:?}", root_dir);
        info!("Entering working directory: {}", &repo);
        env::set_current_dir(repo).unwrap();
        log!(verbose, "Current dir: {:?}", env::current_dir().unwrap());
        log!(verbose, "Pulling");
        Command::new("git")
            .arg("pull")
            .spawn()
            .unwrap()
            .wait()
            .unwrap();

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
    // If no packages are specified, imply all
    let all = packages.is_empty();
    log!(verbose, "All: {}", all);

    // Read repos from config file
    let repos = crate::workspace::read_cfg(verbose)
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
    do_the_pulling(repos_applicable, verbose);
}
