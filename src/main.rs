#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(clippy::too_many_lines)]

use clap::Parser;

use crate::args::{Args, Operation};
use crate::internal::parse_cfg;
use crate::internal::AppExitCode;

#[cfg(target_os = "linux")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

mod args;
mod internal;
mod operations;
mod repository;

fn repository(verbose: bool) -> bool {
    // Parse config
    let config = parse_cfg(verbose);
    log!(verbose, "Config: {:?}", config);

    // Get repository mode status
    let repository = config.base.mode == "repository";
    log!(verbose, "Repository Mode: {:?}", repository);

    // Return repository mode status
    repository
}

fn main() {
    #[cfg(target_os = "linux")]
    if unsafe { libc::geteuid() } == 0 {
        crash!(AppExitCode::RunAsRoot, "Running malachite as root is disallowed as it can lead to system breakage. Instead, malachite will prompt you when it needs superuser permissions");
    }

    // Get required variables
    let args: Args = Args::parse();
    let exclude = &args.exclude;
    let verbose = args.verbose;
    log!(verbose, "Args: {:?}", args);
    log!(verbose, "Exclude: {:?}", exclude);
    log!(verbose, "Verbose: You guess. :)");

    // Arg matching
    match args.subcommand.unwrap_or(Operation::Clone) {
        Operation::Clone => operations::clone(verbose),
        Operation::Build {
            packages, no_regen, ..
        } => {
            if !repository(verbose) {
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
            if !repository(verbose) {
                crash!(
                    AppExitCode::BuildInWorkspace,
                    "Cannot generate repository in workspace mode"
                );
            }
            repository::generate(verbose);
        }
        Operation::Config => operations::config(verbose),
        Operation::Prune => {
            if !repository(verbose) {
                crash!(
                    AppExitCode::BuildInWorkspace,
                    "Cannot prune packages in workspace mode"
                );
            }
            operations::prune(verbose);
        }
        Operation::Clean { force, .. } => operations::clean(verbose, force),
        Operation::Info => operations::info(verbose),
    }
}
