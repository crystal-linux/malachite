use crate::internal::structs::{ErroredPackage, Repo};
use crate::internal::AppExitCode;
use crate::{crash, info, log, repository, workspace};

pub fn build(packages: Vec<String>, exclude: Vec<String>, no_regen: bool) {
    // Read config struct from mlc.toml
    let config = workspace::read_cfg();
    log!("Config: {:?}", config);
    let all = packages.is_empty();
    log!("All: {:?}", all);
    log!("Signing: {:?}", config.sign);

    // Get list of repos and subtract exclude
    let mut repos: Vec<Repo> = config.repo;
    log!("{} Repos: {:?}", repos.len(), repos );
    if !exclude.is_empty() {
        log!("Exclude not empty: {:?}", exclude);
        for ex in exclude {
            repos.retain(|x| *x.name != ex);
        }
    }

    log!("Exclusions parsed. Now {} Repos: {:?}", repos.len(), repos);

    // If packages is not empty and all isn't specified, build specified packages
    let mut errored: Vec<ErroredPackage> = vec![];
    if !packages.is_empty() && !all {
        log!("Packages not empty: {:?}", packages);
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
                log!("Building {}", pkg);
                let code = repository::build(pkg, config.sign);
                log!("Package {} finished with exit code: {:?}", pkg, code);
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
        log!("Proceeding to build all");

        // Sort by package priority
        log!("Sorting by priority: {:?}", repos);
        repos.sort_by(|a, b| b.priority.cmp(&a.priority));
        log!("Sorted: {:?}", repos);
        for pkg in repos {
            log!("Building {}", pkg.name);
            let code = repository::build(&pkg.name, config.sign);
            log!("Package {} finished with exit code: {:?}", pkg.name, code);
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
        log!("Packages empty. Crashing");
        crash!(AppExitCode::NoPkgs, "No packages specified");
    }

    // If no_regen is passed, do not generate a repository
    if !no_regen {
        log!("Generating repository");
        repository::generate();
    }

    // Map errored packages to a string for display
    let error_strings: Vec<String> = errored
        .iter()
        .map(|x| format!("{}: Returned {}", x.name, x.code))
        .collect();

    // If errored is not empty, let the user know which packages failed
    if !errored.is_empty() {
        log!("Errored packages: \n{:?}", error_strings);
        info!(
            "The following packages build jobs returned a non-zero exit code: {}",
            error_strings.join("\n")
        )
    }
}
