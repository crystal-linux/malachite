use crate::internal::structs::{ErroredPackage, Repo};
use crate::internal::AppExitCode;
use crate::{crash, info, repository, workspace};

pub fn build(packages: Vec<String>, exclude: Vec<String>, no_regen: bool) {
    // Read config struct from mlc.toml
    let config = workspace::read_cfg();
    let all = packages.is_empty();

    // Get list of repos and subtract exclude
    let mut repos: Vec<Repo> = config.repo;
    if !exclude.is_empty() {
        for ex in exclude {
            repos.retain(|x| *x.name != ex);
        }
    }

    // If packages is not empty and all isn't specified, build specified packages
    let mut errored: Vec<ErroredPackage> = vec![];
    if !packages.is_empty() && !all {
        for pkg in &packages {
            // If repo is not in config, crash
            if !repos.iter().map(|x| x.name.clone()).any(|x| x == *pkg) {
                crash!(
                    AppExitCode::PkgNotFound,
                    "Package repo {} not found in in mlc.toml",
                    pkg
                );
            } else {
                // Otherwise, build
                let code = repository::build(pkg, config.sign);
                if code != 0 {
                    let error = ErroredPackage {
                        name: pkg.to_string(),
                        code,
                    };
                    errored.push(error);
                }
            }
        }
    }

    // If all is specified, attempt to build a package from all repos
    if all {
        // Sort by package priority
        repos.sort_by(|a, b| b.priority.cmp(&a.priority));
        for pkg in repos {
            let code = repository::build(&pkg.name, config.sign);
            if code != 0 {
                let error = ErroredPackage {
                    name: pkg.name,
                    code,
                };
                errored.push(error);
            }
        }
    }

    // If all is not specified, but packages is empty, crash
    if !all && packages.is_empty() {
        crash!(AppExitCode::NoPkgs, "No packages specified");
    }

    // If no_regen is passed, do not generate a repository
    if !no_regen {
        repository::generate();
    }

    // Map errored packages to a string for display
    let error_strings: Vec<String> = errored
        .iter()
        .map(|x| format!("{}: Returned {}", x.name, x.code))
        .collect();

    // If errored is not empty, let the user know which packages failed
    if !errored.is_empty() {
        info!(
            "The following packages build jobs returned a non-zero exit code: {}",
            error_strings.join("\n")
        )
    }
}
