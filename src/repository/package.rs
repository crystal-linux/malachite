use std::path::Path;
use std::process::Command;
use std::{env, fs};

use crate::{crash, log};
use crate::internal::AppExitCode;

pub fn build(pkg: &str, sign: bool) -> i32 {
    log!("Building {}", pkg);
    log!("Signing: {}", sign);
    // Set root dir to return after build
    let dir = env::current_dir().unwrap();
    log!("Root dir: {:?}", dir);

    // Create out dir if not already present
    if !Path::exists("out".as_ref()) {
        log!("Creating out dir");
        fs::create_dir_all("out").unwrap();
    }

    // If package directory is not found, crash
    if !Path::exists(pkg.as_ref()) {
        crash!(
            AppExitCode::RepoNotFound,
            "Repo for {} not found, aborting",
            pkg
        );
    }

    // Enter build directory
    env::set_current_dir(pkg).unwrap();
    log!("Current dir: {:?}", env::current_dir().unwrap());

    // Build each package
    let a = Command::new("makepkg")
        .args(&[
            "-sf",
            "--skippgpcheck",
            if sign { "--sign" } else { "--nosign" },
            "--noconfirm",
        ])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
    log!("{} Build job returned: {:?}", pkg, a);

    // Copy built package to out dir
    Command::new("bash")
        .args(&["-c", "cp *.pkg.tar* ../out/"])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
    log!("Copied built package to out dir");

    // Return to root dir
    env::set_current_dir(dir).unwrap();
    log!("Returned to root dir: {:?}", env::current_dir().unwrap());

    // Return exit code
    a.code().unwrap()
}
