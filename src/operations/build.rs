use crate::repository::generate;
use crate::{crash, info, repository, workspace};
use crate::internal::AppExitCode;

pub fn build(packages: Vec<String>, exclude: Vec<String>, no_regen: bool) {
    let all = packages.is_empty();

    let config = workspace::read_cfg();

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
            crash!(AppExitCode::PkgNotFound, "Package {} not found in repos in mlc.toml", pkg);
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
        info!(
            "The following packages build jobs returned a non-zero exit code: {}",
            errored.join(" ")
        )
    }
}
