use crate::internal::AppExitCode;
use crate::{crash, info, workspace};
use std::process::Command;

pub fn init() {
    let config = workspace::read_cfg();
    if config.mode == "workspace" || config.mode == "repository" {
        let dirs_raw = Command::new("ls").arg("-1").output().unwrap().stdout;
        let dirs_string = String::from_utf8_lossy(&dirs_raw);
        let mut dirs = dirs_string.lines().collect::<Vec<&str>>();

        dirs.retain(|x| *x != "mlc.toml");
        dirs.sort_unstable();

        let repos = &config.repo;
        let mut repos = repos
            .iter()
            .map(|x| x.split('/').last().unwrap())
            .collect::<Vec<&str>>();

        repos.sort_unstable();

        let mut diff = repos.clone();
        diff.retain(|x| !dirs.contains(x));

        let mut diff_matches = vec![];

        for &x in &diff {
            for y in config.repo.iter() {
                if x == y.split('/').last().unwrap() {
                    diff_matches.push(y);
                }
            }
        }

        if diff.is_empty() {
            info!("All repos are already cloned");
        } else {
            info!("New/missing repos to clone: {}", diff.join(", "));
            for r in diff_matches {
                info!(
                    "Cloning ({} mode): {}",
                    config.mode,
                    r.split('/').last().unwrap()
                );
                Command::new("git")
                    .args(&["clone", r])
                    .spawn()
                    .unwrap()
                    .wait()
                    .unwrap();
            }
        }
    } else {
        crash!(AppExitCode::InvalidMode, "Invalid mode in mlc.toml");
    }
}
