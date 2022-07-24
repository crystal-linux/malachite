use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::internal::AppExitCode;
use crate::{crash, log};

const DEFAULT_CONFIG: &str = r#"
[base]
# Either "repository" or "workspace"
mode = ""
# Better left as true, but can be set to false if it causes issues with branches
smart_pull = true

[mode.repository]
# Decides what to call the repository and relevant files
name = ""
# Decides whether to build packages if package repo is updated on pull
build_on_update = false

[mode.repository.signing]
# Decides whether or not to sign packages
enabled = true

[mode.workspace]
# There are currently no options for workspace mode

[repositories]
# List of repositories formatted as id:name (priority is decided by the ! suffix, and decides package build order)
repos = [
    "aur:hello!",
    "crs:malachite"
]

[repositories.urls]
# URL keys for repositories, with {} where the repository name would go
crs = "https://github.com/crystal-linux/{}"
aur = "https://aur.archlinux.org/{}"
"#;

pub fn create(verbose: bool) {
    // Ensure current directory is empty
    if env::current_dir()
        .unwrap()
        .read_dir()
        .unwrap()
        .next()
        .is_some()
    {
        crash!(
            AppExitCode::DirNotEmpty,
            "Directory is not empty, please only create a repository in an empty directory"
        );
    }
    log!(verbose, "Creating config file");

    // If config file exists, create it
    if !Path::exists("mlc.toml".as_ref()) {
        let mut file = File::create("mlc.toml").unwrap();
        file.write_all(DEFAULT_CONFIG.as_ref()).unwrap();
    }
    log!(verbose, "Config file created");
}
