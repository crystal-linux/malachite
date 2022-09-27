use crate::{crash, info, internal::AppExitCode, log};

pub fn clean(verbose: bool, force: bool) {
    info!("Resetting mlc repo, deleting all directories");
    // Get a vec of all files/dirs in the current directory
    let dir_paths = std::fs::read_dir(".").unwrap();
    log!(verbose, "Paths: {:?}", dir_paths);
    let mut dirs = dir_paths
        .map(|x| x.unwrap().path().display().to_string())
        .collect::<Vec<String>>();

    // Remove mlc.toml and .git from output
    dirs.retain(|x| *x != "./mlc.toml" && *x != ".\\mlc.toml");
    dirs.retain(|x| *x != "./.git" && *x != ".\\.git");
    dirs.retain(|x| *x != "./.gitignore" && *x != ".\\.gitignore");
    dirs.retain(|x| *x != "./.gitmodules" && *x != ".\\.gitmodules");
    dirs.retain(|x| *x != "./README.md" && *x != ".\\README.md");

    let mut unclean_dirs = vec![];

    // Enter each directory and check git status
    for dir in &dirs {
        let root_dir = std::env::current_dir().unwrap();

        log!(verbose, "Entering directory: {}", dir);
        std::env::set_current_dir(dir).unwrap();

        let status = std::process::Command::new("git")
            .arg("status")
            .output()
            .unwrap();

        let output = std::string::String::from_utf8(status.stdout).unwrap();
        log!(verbose, "Git status: {}", output);

        if output.contains("Your branch is up to date with")
            && !output.contains("Untracked files")
            && !output.contains("Changes not staged for commit")
        {
            log!(verbose, "Directory {} is clean", dir);
        } else {
            unclean_dirs.push(dir);
        }

        std::env::set_current_dir(&root_dir).unwrap();
        log!(verbose, "Current directory: {}", root_dir.display());
    }

    if !unclean_dirs.is_empty() && !force && crate::parse_cfg(verbose).base.mode == "workspace" {
        crash!(
            AppExitCode::RepoNotClean,
            "The following directories are not clean: \n   {}\n\
            If you are sure no important changes are staged, run `mlc clean` with the `--force` flag to delete them.",
            unclean_dirs.iter().map(|x| (*x).to_string().replace("./", "").replace(".\\", "")).collect::<Vec<String>>().join(", ")
        );
    }

    log!(verbose, "Paths with mlc.toml excluded: {:?}", dirs);
    for dir in &dirs {
        log!(verbose, "Deleting directory: {}", dir);
        rm_rf::remove(dir).unwrap();
    }
    info!(
        "Reset complete, dirs removed: \n  \
        {}",
        dirs.iter()
            .map(|x| x.replace("./", "").replace(".\\", ""))
            .collect::<Vec<String>>()
            .join("\n  ")
    );
}
