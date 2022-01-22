use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use crate::crash;

const DEFAULT_CONFIG: &str = r#"mode = ""   # either "repository" or "workspace"
name = ""   # only required when in repository mode, decides what to call the repository and relevant files
repo = [""] # an array of git repos to clone from"#;

pub fn create_config() {
    if env::current_dir().unwrap().read_dir().unwrap().next().is_some() {
        crash("Directory is not empty, please only create a repository in an empty directory".to_string(), 6);
    }
    if !Path::exists("mlc.toml".as_ref()) {
        let mut file = File::create("mlc.toml").unwrap();
        file.write_all(DEFAULT_CONFIG.as_ref()).unwrap();
    }
}