use std::env;
use std::fs;
use std::path::PathBuf;

use crate::info;
use crate::log;
use crate::parse_cfg;

#[derive(Debug, Clone)]
struct PackageFile {
    name: String,
    ver: String,
    ext: String,
}

pub fn prune(verbose: bool) {
    // Read config struct from mlc.toml
    let config = parse_cfg(verbose);
    log!(verbose, "Config: {:?}", config);

    // Read current directory
    let current_dir = env::current_dir().unwrap();
    log!(verbose, "Current dir: {:?}", current_dir);

    // Enter out directory
    env::set_current_dir("out").unwrap();
    log!(verbose, "Current dir: {:?}", env::current_dir().unwrap());

    // Read all files from ./ into a Vec<PathBuf>, except for .sig files
    let mut files: Vec<PathBuf> = vec![];
    for entry in fs::read_dir("./").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.extension().unwrap() != "sig" {
            files.push(path);
        }
    }
    log!(verbose, "Files: {:?}", files);

    // Split files into Vec<PackageFile>, turning name-1.0.0-1-x86_64.tar.gz into PackageFile { name: "name", ver: "1.0.0-1", ext: "x86_64.tar.gz" }
    let mut packages: Vec<PackageFile> = vec![];
    for file in files {
        // Regex fuckery. Please don't mess with this.
        let re = regex::Regex::new(r"^(.+)(-.+-.+)(-.+\..+\..+\.+..+)$").unwrap();
        let file = file.to_str().unwrap();
        for cap in re.captures_iter(file) {
            let name = cap[1].to_string();
            let mut ver = cap[2].to_string();
            // Remove the leading "-" from the version and ext strings
            ver.remove(0).to_string();
            let mut ext = cap[3].to_string();
            ext.remove(0).to_string();
            let package = PackageFile { name, ver, ext };
            log!(verbose, "Package: {:?}", package);
            packages.push(package);
        }
    }

    // Split packages into a Vector of Vectors by unique name
    let mut packages_by_name: Vec<Vec<&PackageFile>> = vec![];
    for package in &packages {
        log!(verbose, "Sorting Package: {:?}", package);
        let name = &package.name;
        let mut found = false;
        for p in &mut packages_by_name {
            if &p[0].name == name {
                log!(verbose, "Found {}", name);
                found = true;
                p.push(package);
            }
        }
        if !found {
            log!(verbose, "Creating {}", name);
            packages_by_name.push(vec![package]);
        }
    }

    // Sort each Vector of Vectors by version
    for p in &mut packages_by_name {
        log!(verbose, "Sorting {:?}", p);
        p.sort_by(|a, b| b.ver.cmp(&a.ver));
    }

    // Pushes all but the 4 most recent versions of each package into a new Vector of PackageFiles
    let mut packages_to_delete: Vec<PackageFile> = vec![];
    for p in &packages_by_name {
        let mut to_delete = vec![];
        for (i, _) in p.iter().enumerate() {
            if i >= 3 {
                log!(verbose, "Deleting {:?}", p[i]);
                to_delete.push(p[i].clone());
            }
        }
        packages_to_delete.extend(to_delete);
    }
    log!(verbose, "Packages to delete: {:?}", packages_to_delete);

    // Delete all packages in packages_to_delete
    for p in &packages_to_delete {
        let path = format!("{}-{}-{}", p.name, p.ver, p.ext);
        log!(verbose, "Deleting {}", path);
        std::process::Command::new("bash")
            .args(&["-c", &format!("rm -rf ./{} ./{}.sig", path, path)])
            .output()
            .unwrap();
    }

    // Return to current directory
    env::set_current_dir(current_dir).unwrap();
    log!(verbose, "Current dir: {:?}", env::current_dir().unwrap());

    // Print which packages were deleted
    if packages_to_delete.is_empty() {
        info!("No packages were deleted.");
    } else {
        info!("Deleted the following packages:");
        for p in &mut packages_to_delete {
            info!("{}-{}", p.name.replace("./", ""), p.ver);
        }
    }
}
