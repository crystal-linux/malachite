use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub mode: String,
    pub sign: bool,
    pub name: Option<String>,
    pub repo: Vec<Repo>,
}

#[derive(Debug, Deserialize)]
pub struct Repo {
    pub name: String,
    pub url: String,
    pub priority: usize,
}

#[derive(Debug, Deserialize)]
pub struct UnexpandedConfig {
    pub mode: String,
    pub sign: bool,
    pub name: Option<String>,
    pub repo: Vec<String>,
    pub urls: Vec<String>,
}

#[derive(Debug)]
pub struct SplitRepo {
    pub indx: usize,
    pub name: String,
}

#[derive(Debug)]
pub struct ErroredPackage {
    pub name: String,
    pub code: i32,
}
