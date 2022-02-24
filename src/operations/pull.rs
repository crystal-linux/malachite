use crate::info;
use clap::ArgMatches;
use std::env;
use std::process::Command;

pub fn pull(matches: &ArgMatches) {
    let packages: Vec<String> = matches
        .subcommand_matches("pull")
        .unwrap()
        .values_of_lossy("package(s)")
        .unwrap_or_default();

    if packages.is_empty() {
        let stdout = Command::new("ls").arg("-1").output().unwrap().stdout;
        let dirs_string = String::from_utf8_lossy(&stdout);

        let mut dirs = dirs_string.lines().collect::<Vec<&str>>();

        dirs.retain(|x| *x != "mlc.toml");

        for dir in dirs {
            let cdir = env::current_dir().unwrap();
            info(format!("Entering working directory: {}", dir));
            env::set_current_dir(dir).unwrap();
            Command::new("git")
                .arg("pull")
                .spawn()
                .unwrap()
                .wait()
                .unwrap();
            env::set_current_dir(cdir).unwrap();
        }
    } else {
        for dir in packages {
            let cdir = env::current_dir().unwrap();
            info(format!("Entering working directory: {}", dir));
            env::set_current_dir(dir).unwrap();
            Command::new("git")
                .arg("pull")
                .spawn()
                .unwrap()
                .wait()
                .unwrap();
            env::set_current_dir(cdir).unwrap();
        }
    }
}
