use crate::{internal::AppExitCode, crash};
use std::env;
use std::process::Command;

use crate::info;

fn do_the_pulling(repos: Vec<String>) {
    for repo in repos {
        // Set root dir to return after each git pull
        let root_dir = env::current_dir().unwrap();
        info!("Entering working directory: {}", &repo);
        env::set_current_dir(repo).unwrap();
        Command::new("git")
            .arg("pull")
            .spawn()
            .unwrap()
            .wait()
            .unwrap();

        // Return to root dir
        env::set_current_dir(root_dir).unwrap();
    }
}

pub fn pull(packages: Vec<String>, exclude: Vec<String>) {
    // If no packages are specified, imply all
    let all = packages.is_empty();

    // Read repos from config file
    let repos = crate::workspace::read_cfg()
        .repo
        .iter()
        .map(|x| x.name.clone())
        .collect::<Vec<String>>();

    // Set repos_applicable for next function
    let mut repos_applicable = if all {
        repos
    } else {
        packages
    };

    // Subtract exclude from repos_applicable
    if !exclude.is_empty() {
        for ex in exclude {
            repos_applicable.retain(|x| *x != ex);
        }
    }

    // If all is not specified and packages is empty, crash
    if repos_applicable.is_empty() {
        crash!(AppExitCode::NoPkgs, "No packages specified");
    }

    // Pull!
    do_the_pulling(repos_applicable);
}
