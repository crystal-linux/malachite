use serde_derive::Deserialize;

//// Config structs
#[derive(Debug, Deserialize)]
pub struct Config {
    pub mode: String,
    pub name: String,
    pub sign: bool,
    pub smart_pull: bool,
    pub build_on_update: bool,
    pub repo: Vec<Repo>,
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
    pub sign: bool,
    pub build_on_update: bool,
}

#[derive(Debug, Deserialize)]
pub struct ConfigModeWorkspace {}

#[derive(Debug, Deserialize)]
pub struct ConfigRepositories {
    pub name: Vec<String>,
    pub urls: Vec<String>,
}

//// Repository structs
#[derive(Debug, Deserialize)]
pub struct Repo {
    pub name: String,
    pub url: String,
    pub priority: usize,
}

#[derive(Debug)]
pub struct SplitRepo {
    pub indx: usize,
    pub name: String,
}

//// Build operation structs
#[derive(Debug)]
pub struct ErroredPackage {
    pub name: String,
    pub code: i32,
}
