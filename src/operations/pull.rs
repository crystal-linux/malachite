use std::env;
use std::process::Command;

use crate::info;
use crate::{crash, internal::AppExitCode, log};

struct PullParams {
    smart_pull: bool,
    build_on_update: bool,
    no_regen: bool,
    no_deps: bool,
}

fn do_the_pulling(repos: Vec<String>, verbose: bool, params: &PullParams) {
    for repo in repos {
        // Set root dir to return after each git pull
        let root_dir = env::current_dir().unwrap();
        log!(verbose, "Root dir: {:?}", root_dir);

        // Enter repo dir
        info!("Entering working directory: {}", &repo);
        env::set_current_dir(&repo).unwrap();
        log!(verbose, "Current dir: {:?}", env::current_dir().unwrap());

        let mut packages_to_rebuild: Vec<String> = vec![];

        // Pull logic
        log!(verbose, "Pulling");
        if params.smart_pull {
            // Update the remote
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
                info!("Branch out of date, pulling changes");
                Command::new("git")
                    .arg("pull")
                    .spawn()
                    .unwrap()
                    .wait()
                    .unwrap();

                // If build_on_update is set, rebuild package
                if params.build_on_update {
                    info!("Package {} updated, staging for rebuild", &repo);
                    log!(verbose, "Pushing package {} to be rebuilt", &repo);
                    packages_to_rebuild.push(repo);
                }
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
        env::set_current_dir(&root_dir).unwrap();
        log!(
            verbose,
            "Returned to root dir: {:?}",
            env::current_dir().unwrap()
        );

        // Rebuild packages if necessary
        if !packages_to_rebuild.is_empty() && params.build_on_update {
            info!("Rebuilding packages: {}", &packages_to_rebuild.join(", "));
            log!(verbose, "Rebuilding packages: {:?}", &packages_to_rebuild);

            // Push to build
            crate::operations::build(&packages_to_rebuild, vec![], params.no_regen, verbose, params.no_deps);
            
            // Ensure you are in root dir
            env::set_current_dir(root_dir).unwrap();
            log!(
                verbose,
                "Returned to root dir: {:?}",
                env::current_dir().unwrap()
            );
        }
    }
}

pub fn pull(packages: Vec<String>, exclude: &[String], verbose: bool, no_regen: bool, no_deps: bool) {
    // Read config file
    let config = crate::parse_cfg(verbose);
    log!(verbose, "Config: {:?}", config);

    // If no packages are specified, imply all
    let all = packages.is_empty();
    log!(verbose, "All: {}", all);

    // Read smart_pull from config
    let smart_pull = config.base.smart_pull;
    log!(verbose, "Smart pull: {}", smart_pull);

    // Read build_on_update from config
    let build_on_update = if config.mode.repository.is_some() {
        config.mode.repository.unwrap().build_on_update
    } else {
        false
    };
    log!(verbose, "Build on update: {}", build_on_update);

    // Read repos from config
    let repos = config
        .repositories
        .iter()
        .map(|x| x.name.clone())
        .collect::<Vec<String>>();
    log!(verbose, "Repos: {:?}", repos);

    // Set repos_applicable for next function
    let mut repos_applicable = if all { repos } else { packages };
    log!(verbose, "Repos applicable: {:?}", repos_applicable);

    // Subtract exclude from repos_applicable
    if !exclude.is_empty() {
        for ex in exclude.iter() {
            repos_applicable.retain(|x| *x != *ex);
        }
    }
    log!(verbose, "Exclude: {:?}", exclude);
    log!(verbose, "Repos applicable excluded: {:?}", repos_applicable);

    // If all is not specified and packages is empty, crash
    if repos_applicable.is_empty() {
        crash!(AppExitCode::PkgsNotFound, "No packages specified");
    }

    // Sort repos_applicable by priority
    repos_applicable.sort_by(|a, b| {
        config
            .repositories
            .iter()
            .find(|x| x.name == *a)
            .unwrap()
            .priority
            .cmp(
                &config
                    .repositories
                    .iter()
                    .find(|x| x.name == *b)
                    .unwrap()
                    .priority,
            )
    });

    log!(verbose, "Pulling {:?}", repos_applicable);

    // If any repos are not in the config, run a clone
    for repo in &repos_applicable {
        if !std::path::Path::new(repo).exists() {
            info!(
                "Repo {} does not exist, ensuring all repos are cloned",
                repo
            );
            crate::operations::clone(verbose);
            break;
        }
    }

    // Pull!
    do_the_pulling(
        repos_applicable,
        verbose,
        &PullParams {
            smart_pull,
            build_on_update,
            no_regen,
            no_deps,
        },
    );
}
