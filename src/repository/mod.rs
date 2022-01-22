mod package;
mod repo;
mod config;

pub fn build(pkg: String) {
    package::build(pkg);
}

pub fn generate() {
    repo::generate();
}

pub fn create_config() {
    config::create_config();
}