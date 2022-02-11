use crate::{crash, info, workspace};
use std::process::Command;

pub fn reinit() {
    let config = workspace::read_cfg();
    let out = Command::new("bash")
        .args(&["-c", "ls -A"])
        .output()
        .unwrap()
        .stdout;
    let dirs_to_s = String::from_utf8_lossy(&*out);
    let mut dirs = dirs_to_s.lines().collect::<Vec<&str>>();

    let name = config.name.unwrap();

    dirs.retain(|x| *x != "mlc.toml");
    dirs.retain(|x| *x != ".git");
    if config.mode == "repository" {
        dirs.retain(|x| *x != "out");
        dirs.retain(|x| *x != name);
    }

    info("Removing all repo directories to reinitialise".to_string());

    Command::new("rm")
        .args(&["-rf", &dirs.join(" ")])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    if config.mode == "workspace" {
        for r in config.repo {
            info(format!("Cloning (workspace mode): {}", r));
            Command::new("git")
                .args(&["clone", &r])
                .spawn()
                .unwrap()
                .wait()
                .unwrap();
        }
    } else if config.mode == "repository" {
        for r in config.repo {
            info(format!("Cloning (repository mode): {}", r));
            Command::new("git")
                .args(&["clone", &r])
                .spawn()
                .unwrap()
                .wait()
                .unwrap();
        }
    } else {
        crash("Invalid mode in mlc.toml".to_string(), 1);
    }
}

pub fn init() {
    let config = workspace::read_cfg();
    if config.mode == "workspace" {
        for r in config.repo {
            info(format!("Cloning (workspace mode): {}", r));
            Command::new("git")
                .args(&["clone", &r])
                .spawn()
                .unwrap()
                .wait()
                .unwrap();
        }
    } else if config.mode == "repository" {
        for r in config.repo {
            info(format!("Cloning (repository mode): {}", r));
            Command::new("git")
                .args(&["clone", &r])
                .spawn()
                .unwrap()
                .wait()
                .unwrap();
        }
    } else {
        crash("Invalid mode in mlc.toml".to_string(), 1);
    }
}
