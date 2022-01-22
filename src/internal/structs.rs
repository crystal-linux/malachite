use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub mode: String,
    pub name: Option<String>,
    pub repo: Vec<String>,
}
