use crate::internal::structs::{ErroredPackage, Repo};
use crate::internal::AppExitCode;
use crate::{crash, info, log, repository};

pub fn build(packages: &[String], exclude: Vec<String>, no_regen: bool, verbose: bool) {
    // Read config struct from mlc.toml
    let config = crate::internal::parse_cfg(verbose);
    log!(verbose, "Config: {:?}", config);
    // Check if any packages were passed, if not, imply all
    let all = packages.is_empty();
    log!(verbose, "All: {:?}", all);

    // Parse whether to sign on build or not
    let sign = if config.mode.repository.as_ref().unwrap().signing.enabled
        && config.mode.repository.as_ref().unwrap().signing.on_gen
    {
        false
    } else {
        config.mode.repository.as_ref().unwrap().signing.enabled
    };
    log!(
        verbose,
        "Signing: {:?}",
        config.mode.repository.unwrap().signing
    );

    // Get list of repos and subtract exclude
    let mut repos: Vec<Repo> = config.repositories;
    log!(verbose, "{} Repos: {:?}", repos.len(), repos);
    if !exclude.is_empty() {
        log!(verbose, "Exclude not empty: {:?}", exclude);
        for ex in exclude {
            repos.retain(|x| *x.name != ex);
        }
    }

    log!(
        verbose,
        "Exclusions parsed. Now {} Repos: {:?}",
        repos.len(),
        repos
    );

    // If packages is not empty and all isn't specified, build specified packages
    let mut errored: Vec<ErroredPackage> = vec![];
    if !packages.is_empty() && !all {
        log!(verbose, "Packages not empty: {:?}", packages);
        for pkg in packages.iter() {
            // If repo is not in config, crash, otherwise, build
            if repos.iter().map(|x| x.name.clone()).any(|x| x == *pkg) {
                // Otherwise, build
                log!(verbose, "Building {}", pkg);

                let code = repository::build(pkg, sign, verbose);
                log!(
                    verbose,
                    "Package {} finished with exit code: {:?}",
                    pkg,
                    code
                );

                if code != 0 {
                    let error = ErroredPackage {
                        name: pkg.to_string(),
                        code,
                    };
                    errored.push(error);
                }
            } else {
                crash!(
                    AppExitCode::PkgNotFound,
                    "Package repo {} not found in in mlc.toml",
                    pkg
                );
            }
        }
    }

    // If all is specified, attempt to build a package from all repos
    if all {
        log!(verbose, "Proceeding to build all");

        // Sort by package priority
        log!(verbose, "Sorting by priority: {:?}", repos);
        repos.sort_by(|a, b| b.priority.cmp(&a.priority));
        log!(verbose, "Sorted: {:?}", repos);
        for pkg in repos {
            log!(verbose, "Building {}", pkg.name);

            let code = repository::build(&pkg.name, sign, verbose);
            log!(
                verbose,
                "Package {} finished with exit code: {:?}",
                pkg.name,
                code
            );

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
        log!(verbose, "Packages empty. Crashing");
        crash!(AppExitCode::NoPkgs, "No packages specified");
    }

    // If no_regen is passed, do not generate a repository
    if !no_regen {
        log!(verbose, "Generating repository");
        repository::generate(verbose);
    }

    // Map errored packages to a string for display
    let error_strings: Vec<String> = errored
        .iter()
        .map(|x| format!("{}: Returned {}", x.name, x.code))
        .collect();

    // If errored is not empty, let the user know which packages failed
    if !errored.is_empty() {
        log!(verbose, "Errored packages: \n{:?}", error_strings);
        info!(
            "The following packages build jobs returned a non-zero exit code: \n  {}",
            error_strings.join("\n  ")
        );
    }
}
