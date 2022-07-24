use colored::Colorize;
use std::env;
use std::process::Command;
use tabled::Tabled;

use crate::{crash, info, internal::AppExitCode, log};

// For displaying the table of contents
#[derive(Clone, tabled::Tabled, Debug)]
struct RepoDisplayGit {
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "URL")]
    url: String,
    #[tabled(skip)]
    priority: usize,
    #[tabled(rename = "Git Info")]
    git_info: String,
}

#[derive(Clone, tabled::Tabled, Debug)]
struct RepoDisplay {
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "URL")]
    url: String,
    #[tabled(skip)]
    priority: usize,
}

pub fn git_status(verbose: bool, repo: &str, colorblind: bool) -> String {
    let dir = env::current_dir().unwrap();
    log!(
        verbose,
        "Current directory: {}",
        env::current_dir().unwrap().display()
    );
    env::set_current_dir(&repo).unwrap_or_else(|e| {
        crash!(
            AppExitCode::NotInit,
            "Failed to enter directory {} for Git info: {}, Have you initialized the repo?",
            repo,
            e.to_string()
        );
    });
    log!(verbose, "Current directory: {}", repo);

    Command::new("git")
        .args(&["remote", "update"])
        .output()
        .unwrap();

    let output = Command::new("git").arg("status").output().unwrap();
    let output = String::from_utf8(output.stdout).unwrap();
    log!(verbose, "Git status: {}", output);

    let unstaged = output.contains("Changes not staged for commit")
        || output.contains("Changes to be committed");
    let untracked = output.contains("Untracked files");
    let dirty = unstaged || untracked;

    let pull = output.contains("Your branch is behind");
    let push = output.contains("Your branch is ahead");

    let latest_commit = Command::new("git")
        .args(&["log", "--pretty=%h", "-1"])
        .output()
        .unwrap();
    let mut latest_commit = String::from_utf8(latest_commit.stdout).unwrap();
    latest_commit.retain(|c| !c.is_whitespace());

    let output = if colorblind {
        format!(
            "{} {} {} {}",
            if dirty { "D".red() } else { "D".bright_blue() },
            if pull { "Pl".red() } else { "Pl".bright_blue() },
            if push { "Ps".red() } else { "Ps".bright_blue() },
            latest_commit
        )
    } else {
        format!(
            "{} {} {} {}",
            if dirty { "D".red() } else { "D".green() },
            if pull { "Pl".red() } else { "Pl".green() },
            if push { "Ps".red() } else { "Ps".green() },
            latest_commit
        )
    };
    env::set_current_dir(&dir).unwrap();
    log!(verbose, "Current directory: {}", dir.display());
    output
}

pub fn info(verbose: bool) {
    log!(verbose, "Showing Info");
    let config = crate::internal::parse_cfg(verbose);
    log!(verbose, "Config: {:?}", config);

    let git_info = if config.mode.workspace.is_some() {
        config.mode.workspace.as_ref().unwrap().git_info
    } else {
        false
    };
    log!(verbose, "Git info: {}", git_info);

    let colorblind = if config.mode.workspace.is_some() {
        config.mode.workspace.as_ref().unwrap().colorblind
    } else {
        false
    };
    log!(verbose, "Colorblind: {}", colorblind);

    // Add the branch to the name if it's not the default branch for said repository
    let repos_unparsed = config.repositories;
    let mut repos = vec![];
    let mut repos_git = vec![];
    for repo in repos_unparsed {
        // Get name with branch, '/' serving as the delimiter
        let name = if repo.branch.is_some() {
            format!("{}/{}", repo.name, repo.branch.unwrap())
        } else {
            repo.name.clone()
        };

        // Get git info, if applicable
        let git_info_string = if git_info {
            Some(git_status(
                verbose,
                &repo.name,
                config.mode.workspace.as_ref().unwrap().colorblind,
            ))
        } else {
            None
        };

        // Push to the correct vector, we're using a separate vector for git info because
        // the struct we're displaying is different
        if git_info {
            repos_git.push(RepoDisplayGit {
                name,
                url: repo.url.clone(),
                priority: repo.priority,
                git_info: git_info_string.unwrap(),
            });
        } else {
            repos.push(RepoDisplay {
                name,
                url: repo.url.clone(),
                priority: repo.priority,
            });
        }
    }
    log!(verbose, "Repos: {:?}", repos);

    // Sort by priority
    repos.sort_by(|a, b| b.priority.cmp(&a.priority));
    repos_git.sort_by(|a, b| b.priority.cmp(&a.priority));
    if git_info {
        log!(verbose, "Repos Sorted: {:?}", repos_git);
    } else {
        log!(verbose, "Repos Sorted: {:?}", repos);
    }

    // Displaying basic info about the Malachite Repository
    let internal_name = if config.mode.repository.is_none()
        || config.mode.repository.as_ref().unwrap().name.is_empty()
    {
        env::current_dir()
            .unwrap()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
    } else {
        config.mode.repository.unwrap().name
    };
    let name = format!(
        "{} \"{}\":",
        // Sidenote: It should NOT be this convoluted to capitalise the first character of a string in rust. What the fuck.
        String::from_utf8_lossy(&[config.base.mode.as_bytes()[0].to_ascii_uppercase()])
            + &config.base.mode[1..],
        internal_name
    );

    // Get terminal width
    let width = match termion::terminal_size() {
        Ok((w, _)) => w,
        Err(_) => 80,
    };

    // Create table for displaying info
    let table = if git_info {
        tabled::Table::new(&repos_git)
            .with(tabled::Style::modern())
            .with(tabled::Width::wrap(width as usize))
            .to_string()
    } else {
        tabled::Table::new(&repos)
            .with(tabled::Style::modern())
            .with(tabled::Width::wrap(width as usize))
            .to_string()
    };

    // Get length of Vec for displaying in the table
    let len = if git_info {
        repos_git.len()
    } else {
        repos.len()
    };

    // Print all of the info
    info!("{}", name);
    info!("Local Repositories: {}", len);
    println!("{}", table);
    if config.mode.workspace.is_some() && config.mode.workspace.as_ref().unwrap().git_info {
        info!(
            "Key: \n  \
            D:  Dirty -  Unstaged Changes \n  \
            Pl: Pull  -  Changes at Remote \n  \
            Ps: Push  -  Unpushed Changes \n  \
            {}:  Applies, {}: Does Not Apply",
            " ".on_red(),
            if config.mode.workspace.unwrap().colorblind {
                " ".on_bright_blue()
            } else {
                " ".on_green()
            }
        );
    }
}
