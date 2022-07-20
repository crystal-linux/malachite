use crate::crash;
use crate::internal::AppExitCode;
use std::path::Path;
use std::process::Command;
use std::{env, fs};

pub fn build(pkg: &str) -> i32 {
    let dir = env::current_dir().unwrap();
    if !Path::exists("out".as_ref()) {
        fs::create_dir_all("out").unwrap();
    }
    if !Path::exists(pkg.as_ref()) {
        crash!(
            AppExitCode::DirNotGit,
            "Git directory for {} not found, aborting",
            pkg
        );
    }

    env::set_current_dir(pkg).unwrap();

    let a = Command::new("makepkg")
        .args(&["-sf", "--skippgpcheck", "--sign", "--noconfirm"])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    Command::new("bash")
        .args(&["-c", "cp *.pkg.tar* ../out/"])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    env::set_current_dir(dir).unwrap();

    a.code().unwrap()
}
