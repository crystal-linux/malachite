use std::path::Path;
use std::process::Command;
use std::{env, fs};

use crate::internal::AppExitCode;
use crate::{crash, log};

pub fn build(pkg: &str, sign: bool, verbose: bool) -> i32 {
    log!(verbose, "Building {}", pkg);
    log!(verbose, "Signing: {}", sign);

    // Set root dir to return after build
    let dir = env::current_dir().unwrap();
    log!(verbose, "Root dir: {:?}", dir);

    // Create out dir if not already present
    if !Path::exists("out".as_ref()) {
        log!(verbose, "Creating out dir");
        fs::create_dir_all("out").unwrap();
    }

    // If package directory is not found, crash
    if !Path::exists(pkg.as_ref()) {
        crash!(
            AppExitCode::PkgsNotFound,
            "Repo for package {} not found, aborting",
            pkg
        );
    }

    // Enter build directory
    env::set_current_dir(pkg).unwrap();
    log!(verbose, "Current dir: {:?}", env::current_dir().unwrap());

    // If PKGBUILD is not found, return 63 and break
    if !Path::exists("PKGBUILD".as_ref()) {
        env::set_current_dir(&dir).unwrap();
        log!(verbose, "Current dir: {:?}", env::current_dir().unwrap());
        return 63;
    }

    // Parse extra flags from envvar
    let extra_flags = env::var("MAKEPKG_FLAGS").unwrap_or_else(|_| "".to_string());
    let extra_flags = extra_flags.split(' ').collect::<Vec<&str>>();

    // Default set of flags
    let default_args = vec![
        "-sf",
        "--skippgpcheck",
        if sign { "--sign" } else { "--nosign" },
        "--noconfirm",
    ];

    // Build each package
    let a = Command::new("makepkg")
        .args(
            default_args
                .iter()
                .chain(extra_flags.iter())
                .map(std::string::ToString::to_string),
        )
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
    log!(verbose, "{} Build job returned: {:?}", pkg, a);

    // Copy built package to out dir
    Command::new("bash")
        .args(&["-c", "cp *.pkg.tar* ../out/"])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
    log!(verbose, "Copied built package to out dir");

    // Return to root dir
    env::set_current_dir(dir).unwrap();
    log!(
        verbose,
        "Returned to root dir: {:?}",
        env::current_dir().unwrap()
    );

    // Return exit code
    a.code().unwrap()
}
