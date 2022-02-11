use clap::ArgMatches;

mod build;
mod config;
mod init;
mod pull;

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
