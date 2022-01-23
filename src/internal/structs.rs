use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub mode: String,
    pub name: Option<String>,
    pub repo: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct UnexpandedConfig {
    pub mode: String,
    pub name: Option<String>,
    pub repo: Vec<String>,
    pub urls: Vec<String>,
}

#[derive(Debug)]
pub struct SplitRepo {
    pub indx: usize,
    pub name: String
}