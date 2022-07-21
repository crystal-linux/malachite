use std::path::Path;
use std::process::Command;
use std::{env, fs};

use crate::crash;
use crate::internal::AppExitCode;

pub fn build(pkg: &str, sign: bool) -> i32 {
    // Set root dir to return after build

    let dir = env::current_dir().unwrap();

    // Create out dir if not already present
    if !Path::exists("out".as_ref()) {
        fs::create_dir_all("out").unwrap();
    }

    // If directory is not found, crash
    if !Path::exists(pkg.as_ref()) {
        crash!(
            AppExitCode::RepoNotFound,
            "Repo for {} not found, aborting",
            pkg
        );
    }

    // Enter build directory
    env::set_current_dir(pkg).unwrap();

    // Build each package
    let a = Command::new("makepkg")
        .args(&["-sf", "--skippgpcheck", if sign { "--sign" } else {"--nosign"}, "--noconfirm"])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    // Copy built package to out dir
    Command::new("bash")
        .args(&["-c", "cp *.pkg.tar* ../out/"])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    // Return to root dir
    env::set_current_dir(dir).unwrap();

    // Return exit code
    a.code().unwrap()
}
