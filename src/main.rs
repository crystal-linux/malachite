#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
)]

use clap::Parser;
use std::env;
use std::path::Path;
use std::process::Command;

use crate::args::{Args, Operation};
use crate::internal::AppExitCode;
use crate::internal::parse_cfg;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

mod args;
mod internal;
mod operations;
mod repository;

fn main() {
    if unsafe { libc::geteuid() } == 0 {
        crash!(AppExitCode::RunAsRoot, "Running malachite as root is disallowed as it can lead to system breakage. Instead, malachite will prompt you when it needs superuser permissions");
    }

    let args: Args = Args::parse();
    let exclude = &args.exclude;
    let verbose = args.verbose;
    log!(verbose, "Args: {:?}", args);
    log!(verbose, "Exclude: {:?}", exclude);
    log!(verbose, "Verbose: You guess. :)");

    let config = parse_cfg(verbose);
    log!(verbose, "Config: {:?}", config);

    if Path::exists("../.git".as_ref()) {
        log!(verbose, "Detected parent git repository");
        info!("Parent directory is a git directory, pulling latest mlc.toml. It is advised you run mlc pull/update in all malachite directories");

        let dir = env::current_dir().unwrap();
        env::set_current_dir("../").unwrap();
        log!(verbose, "Current dir: {:?}", env::current_dir().unwrap());

        if config.base.smart_pull {
            log!(verbose, "Smart pull");
            Command::new("git")
                .args(&["remote", "update"])
                .spawn()
                .unwrap()
                .wait()
                .unwrap();
            let output = Command::new("git").arg("status").output().unwrap();
            if String::from_utf8(output.stdout)
                .unwrap()
                .contains("Your branch is behind")
            {
                info!("Branch out of date, pulling changes");
                Command::new("git")
                    .arg("pull")
                    .spawn()
                    .unwrap()
                    .wait()
                    .unwrap();
            } else {
                info!("No changes to pull");
            }
        } else {
            log!(verbose, "Normal pull");
            Command::new("git")
                .arg("pull")
                .spawn()
                .unwrap()
                .wait()
                .unwrap();
        }
        env::set_current_dir(dir).unwrap();
        log!(verbose, "Current dir: {:?}", env::current_dir().unwrap());
    }

    let repository = config.base.mode == "repository";
    log!(verbose, "Repository Mode: {:?}", repository);

    match args.subcommand.unwrap_or(Operation::Clone) {
        Operation::Clone => operations::clone(verbose),
        Operation::Build {
            packages, no_regen, ..
        } => {
            if !repository {
                crash!(
                    AppExitCode::BuildInWorkspace,
                    "Cannot build packages in workspace mode"
                );
            }
            operations::build(&packages, exclude.clone(), no_regen, verbose);
        }
        Operation::Pull {
            packages, no_regen, ..
        } => operations::pull(packages, exclude, verbose, no_regen),
        Operation::RepoGen => {
            if !repository {
                crash!(
                    AppExitCode::BuildInWorkspace,
                    "Cannot build packages in workspace mode"
                );
            }
            repository::generate(verbose);
        }
        Operation::Config => operations::config(verbose),
        Operation::Prune => operations::prune(verbose),
        Operation::Clean => operations::clean(verbose),
        Operation::Info => operations::info(verbose),
    }
}
