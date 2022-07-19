use crate::crash;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

const DEFAULT_CONFIG: &str = r#"# either "repository" or "workspace"
mode = ""

# only required when in repository mode, decides what to call the repository and relevant files
name = ""

# an array of git repos to clone from, formatted url_index::repo_name, e.g. if you had urls = [ "https://example.com/%repo%" ], 1::package would expand to https://example.com/package
repo = [
    "",
    ""
]

# an array of urls to clone from, in the format https://example.com/%repo% (the %repo% is NOT optional)
urls = [
    "",
    ""
]"#;

pub fn create_config() {
    if env::current_dir()
        .unwrap()
        .read_dir()
        .unwrap()
        .next()
        .is_some()
    {
        crash(
            "Directory is not empty, please only create a repository in an empty directory"
                .to_string(),
            6,
        );
    }
    if !Path::exists("mlc.toml".as_ref()) {
        let mut file = File::create("mlc.toml").unwrap();
        file.write_all(DEFAULT_CONFIG.as_ref()).unwrap();
    }
}
