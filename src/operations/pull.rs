use crate::info;
use std::env;
use std::process::Command;

fn do_the_pulling(packages: Vec<String>) {
    for dir in packages {
        let current_dir = env::current_dir().unwrap();
        info!("Entering working directory: {}", dir);
        env::set_current_dir(dir).unwrap();
        Command::new("git")
            .arg("pull")
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
        env::set_current_dir(current_dir).unwrap();
    }
}

pub fn pull(packages: Vec<String>, exclude: Vec<String>) {
    let all = packages.is_empty();
    if all {
        let stdout = Command::new("ls").arg("-1").output().unwrap().stdout;
        let dirs_string = String::from_utf8_lossy(&stdout);

        let mut dirs = dirs_string.lines().collect::<Vec<&str>>();

        dirs.retain(|x| *x != "mlc.toml");
        for x in exclude {
            dirs.retain(|y| *y != x);
        }

        let dirs_mapped = dirs.iter().map(|x| x.to_string()).collect();

        do_the_pulling(dirs_mapped);
    } else {
        do_the_pulling(packages);
    }
}
