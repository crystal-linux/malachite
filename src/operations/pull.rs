use std::env;
use std::process::Command;
use clap::ArgMatches;
use crate::{info, workspace};

pub fn pull(matches: &ArgMatches) {
    let packages: Vec<String> = matches
        .subcommand_matches("pull")
        .unwrap()
        .values_of_lossy("package(s)")
        .unwrap_or_default();
    let config = workspace::read_cfg();
    let cdir = env::current_dir().unwrap();
    if packages.is_empty() {
        for r in config.repo {
            info(format!("Entering working directory: {}", r));
            let dir = format!(
                "{}/{}",
                env::current_dir().unwrap().display(),
                r.split('/').collect::<Vec<&str>>().last().unwrap()
            );
            env::set_current_dir(dir).unwrap();
            Command::new("git")
                .args(&["pull", &r])
                .spawn()
                .unwrap()
                .wait()
                .unwrap();
            env::set_current_dir(&cdir).unwrap();
        }
    } else {
        for r in packages {
            info(format!("Entering working directory: {}", r));
            let dir = format!(
                "{}/{}",
                env::current_dir().unwrap().display(),
                r.split('/').collect::<Vec<&str>>().last().unwrap()
            );
            env::set_current_dir(dir).unwrap();
            Command::new("git")
                .args(&["pull", &r])
                .spawn()
                .unwrap()
                .wait()
                .unwrap();
            env::set_current_dir(&cdir).unwrap();
        }
    }
}