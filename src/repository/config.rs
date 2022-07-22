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
# Only required when in repository mode, decides what to call the repository and relevant files
name = ""
# Only required when in repository mode, decides whether to PGP sign built packages
sign = true
# Only required when in repository mode, decides whether to build packages if pull is successful
build_on_update = false

[mode.workspace]
# There are currently no options for workspace mode

[repositories]
# An array of Git repositories to clone from, formatted url_index::repo_name(!)
# e.g. if you had URLs = [ "https://example.com/%repo%.git" ], 1::package would expand to https://example.com/package.git
# Repository mode only: Depending on the number of "!"s appended to the name, the priority of the package will be determined. More "!"s = higher priority = built first.
name = [
    "",
    ""
]

# An array of URLs to clone from, in the format https://example.com/%repo% (the %repo% is NOT optional and will be replaced with the name of the repository)
urls = [
    "",
    ""
]"#;

pub fn create_config(verbose: bool) {
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
