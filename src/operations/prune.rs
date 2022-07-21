use crate::info;
use crate::log;
use crate::read_cfg;
use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone)]
struct PackageFile {
    name: String,
    ver: String,
    ext: String,
}

pub fn prune(verbose: bool) {
    // Read config struct from mlc.toml
    let config = read_cfg(verbose);
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
        let file = file.to_str().unwrap();
        let mut parts = file.split('-');
        let name = parts.next().unwrap();
        let ver = parts.next().unwrap();
        let rel = parts.next().unwrap();
        let ext = parts.next().unwrap();
        let package = PackageFile {
            name: name.to_string(),
            ver: ver.to_string() + "-" + rel,
            ext: ext.to_string(),
        };
        log!(verbose, "Package: {:?}", package);
        packages.push(package);
    }

    // Split packages into a Vector of Vectors by unique name
    let mut packages_by_name: Vec<Vec<&PackageFile>> = vec![];
    for package in &packages {
        log!(verbose, "Sorting Package: {:?}", package);
        let name = &package.name;
        let mut found = false;
        for p in packages_by_name.iter_mut() {
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
    for p in packages_by_name.iter_mut() {
        log!(verbose, "Sorting {:?}", p);
        p.sort_by(|a, b| b.ver.cmp(&a.ver));
    }

    // Pushes all but the 4 most recent versions of each package into a new Vector of PackageFiles
    let mut packages_to_delete: Vec<PackageFile> = vec![];
    for p in packages_by_name.iter() {
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
    for p in packages_to_delete.iter() {
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
    info!("Deleted the following packages:");
    for p in packages_to_delete.iter_mut() {
        info!("{}-{}", p.name.replace("./", ""), p.ver);
    }
}
