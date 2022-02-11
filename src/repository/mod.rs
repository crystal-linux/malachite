mod config;
mod package;
mod repo;

pub fn build(pkg: &str) -> i32 {
    package::build(pkg)
}

pub fn generate() {
    repo::generate();
}

pub fn create_config() {
    config::create_config();
}
