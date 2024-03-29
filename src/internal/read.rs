use std::fs;
use std::path::Path;

use crate::internal::structs::{Config, Repo, SplitRepo, UnexpandedConfig};
use crate::internal::AppExitCode;
use crate::{crash, log};

pub fn parse_cfg(verbose: bool) -> Config {
    // Crash if mlc.toml doesn't exist
    if !Path::exists("mlc.toml".as_ref()) {
        crash!(
            AppExitCode::ConfigParseError,
            "Config file not found (mlc.toml)"
        );
    }

    // Reading the config file to an UnexpandedConfig struct
    let file = fs::read_to_string("mlc.toml").unwrap();
    let config: UnexpandedConfig = toml::from_str(&file).unwrap_or_else(|e| {
        crash!(
            AppExitCode::ConfigParseError,
            "Error parsing config file: {}",
            e
        );
        // This is unreachable, but rustc complains about it otherwise
        std::process::exit(1);
    });

    log!(verbose, "Config file read: {:?}", config);

    // Crash if incorrect mode is set
    if config.base.mode != "workspace" && config.base.mode != "repository" {
        crash!(
            AppExitCode::ConfigParseError,
            "Invalid mode in mlc.toml, must be either \"repository\" or \"workspace\""
        );
    }

    let mut expanded_repos: Vec<Repo> = vec![];

    // Parsing repos from the config file
    for x in config.repositories.repos {
        log!(verbose, "Parsing repo: {:?}", x);
        // Splits the repo name and index into a SplitRepo struct
        let split: Vec<&str> = x.split(':').collect();
        let split_struct = if split.len() > 2 {
            SplitRepo {
                id: split[0].parse().unwrap(),
                name: split[1].parse().unwrap(),
                extra: Some(split[2].parse().unwrap()),
            }
        } else {
            SplitRepo {
                id: split[0].parse().unwrap(),
                name: split[1].parse().unwrap(),
                extra: None,
            }
        };
        log!(verbose, "Split repo: {:?}", split_struct);

        // Parses all necessary values for expanding the repo to a Repo struct
        let id = split_struct.id;

        // If a branch is defined, parse it
        let branch = if split_struct.name.contains('/') {
            log!(verbose, "Branch defined: {}", split_struct.name);
            Some(
                split_struct.name.split('/').collect::<Vec<&str>>()[1]
                    .to_string()
                    .replace('!', ""),
            )
        } else {
            log!(verbose, "No branch defined");
            None
        };

        // Strip branch and priority info from the name, if present
        let name = if split_struct.name.contains('/') {
            split_struct.name.split('/').collect::<Vec<&str>>()[0].to_string()
        } else {
            split_struct.name.to_string().replace('!', "")
        };

        // Substitutes the name into the url
        let urls = &config.repositories.urls;
        let mut urls_vec = vec![];
        for (i, url) in urls {
            if i == &id {
                log!(verbose, "Substituting url: {:?}", url);
                urls_vec.push(url);
            }
        }
        let url = urls_vec[0].replace("{}", &name);

        // Counts instances of ! in the name, and totals a priority accordingly
        let priority = &split_struct.name.matches('!').count();

        // Creates and pushes Repo struct to expanded_repos
        let repo = Repo {
            name,
            url,
            branch,
            extra: split_struct.extra,
            priority: *priority,
        };
        log!(verbose, "Expanded repo: {:?}", repo);
        expanded_repos.push(repo);
    }

    // Returns parsed config file
    let conf = Config {
        base: config.base,
        mode: config.mode,
        repositories: expanded_repos,
    };
    log!(verbose, "Config: {:?}", conf);

    conf
}
