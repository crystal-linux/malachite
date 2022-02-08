use clap::ArgMatches;
use crate::{crash, repository, workspace};

pub fn build(matches: &ArgMatches) {
    let config = workspace::read_cfg();
    let mut packages: Vec<String> = matches
        .subcommand_matches("build")
        .unwrap()
        .values_of_lossy("package(s)")
        .unwrap_or_default();

    let exclude: Vec<String> = matches
        .subcommand_matches("build")
        .unwrap()
        .values_of_lossy("exclude")
        .unwrap_or_default();


    for pkg in &exclude {
        packages.retain(|x| &*x != pkg);
    }

    if config.mode != "repository" {
        crash("Cannot build packages in workspace mode".to_string(), 2);
    }

    let mut repos: Vec<String> = vec![];
    for r in config.repo {
        let split = r.split('/').collect::<Vec<&str>>();
        let a = split.last().unwrap();
        repos.push(a.parse().unwrap());
    }

    if matches
        .subcommand_matches("build")
        .unwrap()
        .is_present("exclude")
    {
        for ex in exclude {
            repos.retain(|x| *x != ex);
        }
    }

    for pkg in packages {
        if !repos.contains(&pkg) {
            crash(format!("Package {} not found in repos in mlc.toml", pkg), 3);
        } else {
            repository::build(pkg);
        }
    }

    if matches
        .subcommand_matches("build")
        .unwrap()
        .is_present("all")
    {
        for pkg in repos {
            repository::build(pkg);
        }
    }

    if matches
        .subcommand_matches("build")
        .unwrap()
        .is_present("regen")
    {
        repository::generate();
    }
}