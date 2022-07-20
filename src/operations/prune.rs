use std::fs;
use std::process::Command;

use crate::{info, read_cfg};

pub fn prune() {
    let config = read_cfg();
    let mut packages = vec![];
    for untrimmed_repo in &config.repo {
        pub fn trim_repo(a: String) -> String {
            (a.split('/')
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .last()
                .unwrap())
            .to_string()
        }
        packages.push(trim_repo(untrimmed_repo.to_string()));
    }

    let mut packages_to_del = vec![];

    for pkg in packages {
        let dups = Command::new("bash")
            .args(&[
                "-c",
                &format!(
                    "ls out/{}*.tar.* -w 1 | grep .sig | sed 's/.sig//g' | sort -r",
                    pkg
                ),
            ])
            .output()
            .unwrap()
            .stdout
            .to_ascii_lowercase();

        let duplicates = String::from_utf8_lossy(&dups);
        let duplicates_lines = duplicates.lines().collect::<Vec<&str>>();
        let variable_hell = duplicates_lines.iter().skip(1).collect::<Vec<&&str>>();

        if !variable_hell.is_empty() {
            for var in variable_hell {
                packages_to_del.push(var.to_string());
            }
        }
    }

    if !packages_to_del.is_empty() {
        info!("Pruning duplicates: {}", packages_to_del.join(", "));
    }

    for pkg in packages_to_del {
        fs::remove_file(&pkg).unwrap();
        fs::remove_file(format!("{}.sig", &pkg)).unwrap();
    }
}
