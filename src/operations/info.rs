use crate::{info, log};
use colored::Colorize;
use std::env;
use tabled::Tabled;

// For displaying the table of contents
#[derive(Clone, tabled::Tabled, Debug)]
struct RepoDisplay {
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "URL")]
    url: String,
    #[tabled(rename = "Priority")]
    priority: usize,
}

pub fn info(verbose: bool) {
    log!(verbose, "Showing Info");
    let config = crate::workspace::read_cfg(verbose);
    log!(verbose, "Config: {:?}", config);

    // Add the branch to the name if it's not the default branch for said repository
    let repos_unparsed = config.repositories;
    let mut repos = vec![];
    for repo in repos_unparsed {
        let name = if repo.branch.is_some() {
            format!("{}/{}", repo.name, repo.branch.unwrap())
        } else {
            repo.name.clone()
        };
        repos.push(RepoDisplay {
            name,
            url: repo.url,
            priority: repo.priority,
        });
    }
    log!(verbose, "Repos: {:?}", repos);
    // Sort by priority
    repos.sort_by(|a, b| b.priority.cmp(&a.priority));
    log!(verbose, "Repos Sorted: {:?}", repos);

    // Displaying basic info about the Malachite Repository
    let internal_name = if !config.mode.repository.name.is_empty() {
        config.mode.repository.name
    } else {
        env::current_dir()
            .unwrap()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
    };
    let name = format!(
        "{} \"{}\":",
        if config.base.mode == "repository" {
            "Repository".to_string()
        } else if config.base.mode == "workspace" {
            "Workspace".to_string()
        } else {
            "".to_string()
        },
        internal_name
    );

    // Get terminal width
    let width = match termion::terminal_size() {
        Ok((w, _)) => w,
        Err(_) => 80,
    };

    // Create table for displaying info
    let table = tabled::Table::new(&repos)
        .with(tabled::Style::modern())
        .with(tabled::Width::wrap(width as usize))
        .to_string();

    // Print all of the info
    info!("{}", name);
    info!(
        "Local Repositories: {}",
        repos.len().to_string().green().bold()
    );
    println!("{}", table.bold());
}
