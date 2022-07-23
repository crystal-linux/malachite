use serde_derive::Deserialize;
use std::collections::HashMap;

//// Config structs
#[derive(Debug, Deserialize)]
pub struct Config {
    pub base: ConfigBase,
    pub mode: ConfigMode,
    pub repositories: Vec<Repo>,
}

#[derive(Debug, Deserialize)]
pub struct UnexpandedConfig {
    pub base: ConfigBase,
    pub mode: ConfigMode,
    pub repositories: ConfigRepositories,
}

#[derive(Debug, Deserialize)]
pub struct ConfigBase {
    pub mode: String,
    pub smart_pull: bool,
}

#[derive(Debug, Deserialize)]
pub struct ConfigMode {
    pub repository: ConfigModeRepository,
    pub workspace: ConfigModeWorkspace,
}

#[derive(Debug, Deserialize)]
pub struct ConfigModeRepository {
    pub name: String,
    pub build_on_update: bool,
    pub signing: ConfigModeRepositorySigning,
}

#[derive(Debug, Deserialize)]
pub struct ConfigModeRepositorySigning {
    pub enabled: bool,
    pub key: String,
    pub on_gen: bool,
}

#[derive(Debug, Deserialize)]
pub struct ConfigModeWorkspace {}

#[derive(Debug, Deserialize)]
pub struct ConfigRepositories {
    pub repos: Vec<String>,
    pub urls: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct ConfigRepositoriesExpanded {
    pub repos: Vec<Repo>,
}

//// Repository structs
#[derive(Debug, Deserialize)]
pub struct Repo {
    pub name: String,
    pub url: String,
    pub branch: Option<String>,
    pub priority: usize,
}

#[derive(Debug)]
pub struct SplitRepo {
    pub id: String,
    pub name: String,
}

//// Build operation structs
#[derive(Debug)]
pub struct ErroredPackage {
    pub name: String,
    pub code: i32,
}
