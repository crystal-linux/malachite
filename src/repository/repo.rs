use std::path::Path;
use std::process::Command;
use std::{env, fs};

use crate::workspace::read_cfg;

pub fn generate() {
    let config = read_cfg();
    let name = config.name.unwrap();

    if Path::exists(name.as_ref()) {
        fs::remove_dir_all(&name).unwrap();
    }

    fs::create_dir_all(&name).unwrap();

    Command::new("bash")
        .args(&["-c", &format!("cp -v out/* {}/", &name)])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    env::set_current_dir(&name).unwrap();

    let db = format!("{}.db", &name);
    let files = format!("{}.files", &name);

    Command::new("bash")
        .args(&["-c", &format!("repo-add {}.tar.gz *.pkg.tar.zst; repo-add {}.tar.gz *.pkg.tar.xz", db)])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    Command::new("bash")
        .args(&["-c", &format!("rm {}.{{db,files}}", &name)])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    Command::new("bash")
        .args(&[
            "-c",
            &format!("mv {}.tar.gz {}; mv {}.tar.gz {}", db, db, files, files),
        ])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}
