use crate::{info, log};

pub fn clean() {
    info!("Resetting mlc repo, deleting all directories");
    // Get a vec of all files/dirs in the current directory
    let dir_paths = std::fs::read_dir("./").unwrap();
    log!("Paths: {:?}", dir_paths);
    let mut dirs = dir_paths
        .map(|x| x.unwrap().path().display().to_string())
        .collect::<Vec<String>>();

    // Remove all files/dirs in the current directory, excluding ./mlc.toml
    dirs.retain(|x| *x != "./mlc.toml");
    log!("Paths with mlc.toml excluded: {:?}", dirs);
    for dir in dirs {
        std::fs::remove_dir_all(dir).unwrap();
    }
}
