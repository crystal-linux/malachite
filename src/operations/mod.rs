use clap::ArgMatches;

mod init;
mod build;
mod pull;
mod config;

pub fn reinit() {
    init::reinit();
}

pub fn init() {
    init::init();
}

pub fn build(matches: &ArgMatches) {
    build::build(matches);
}

pub fn pull(matches: &ArgMatches) {
    pull::pull(matches);
}

pub fn config() {
    config::config();
}