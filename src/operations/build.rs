use crate::repository::generate;
use crate::{crash, info, repository, workspace};

pub fn build(mut packages: Vec<String>, all: bool, exclude: Vec<String>, no_regen: bool) {
    let config = workspace::read_cfg();

    for pkg in &exclude {
        packages.retain(|x| x != pkg);
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

    if exclude.is_empty() {
        for ex in exclude {
            repos.retain(|x| *x != ex);
        }
    }

    let mut errored: Vec<String> = vec![];

    for pkg in packages {
        if !repos.contains(&pkg) {
            crash(format!("Package {} not found in repos in mlc.toml", pkg), 3);
        } else {
            let code = repository::build(&pkg);
            if code != 0 {
                errored.push(pkg);
            }
        }
    }

    if all {
        for pkg in repos {
            let code = repository::build(&pkg);
            if code != 0 {
                errored.push(pkg);
            }
        }
        generate();
    }

    if !no_regen {
        repository::generate();
    }

    if !errored.is_empty() {
        info(format!(
            "The following packages build jobs returned a non-zero exit code: {}",
            errored.join(" ")
        ))
    }
}
