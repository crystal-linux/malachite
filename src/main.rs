use std::path::Path;
use std::process::Command;
use std::{env, fs};

use crate::internal::{crash, info};
use crate::repository::create_config;
use clap::{App, AppSettings, Arg, ArgSettings, SubCommand};

use crate::workspace::read_cfg;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

mod internal;
mod repository;
mod workspace;

fn main() {
    extern "C" {
        fn geteuid() -> u32;
    }

    if unsafe { geteuid() } == 0 {
        crash("Running malachite as root is disallowed as it can lead to system breakage. Instead, malachite will prompt you when it needs superuser permissions".to_string(), 1);
    }

    fn build_app() -> App<'static, 'static> {
        let app = App::new("Malachite")
            .version(env!("CARGO_PKG_VERSION"))
            .about(env!("CARGO_PKG_DESCRIPTION"))
            .arg(
                Arg::with_name("verbose")
                    .short("v")
                    .long("verbose")
                    .multiple(true)
                    .set(ArgSettings::Global)
                    .help("Sets the level of verbosity"),
            )
            .subcommand(
                SubCommand::with_name("build")
                    .about("Builds the given packages")
                    .arg(
                        Arg::with_name("package(s)")
                            .help("The packages to operate on")
                            .multiple(true)
                            .index(1),
                    )
                    .arg(
                        Arg::with_name("all")
                            .long("all")
                            .help("Builds all packages in mlc.toml (except if -x is specified)")
                            .conflicts_with("package(s)"),
                    )
                    .arg(
                        Arg::with_name("exclude")
                            .short("x")
                            .long("exclude")
                            .multiple(true)
                            .takes_value(true)
                            .help("Excludes packages from given operation"),
                    )
                    .arg(
                        Arg::with_name("regen")
                            .short("r")
                            .long("regen")
                            .help("Regenerates repository after building give package(s)"),
                    ),
            )
            .subcommand(
                SubCommand::with_name("repo-gen").about("Generates repository from built packages"),
            )
            .subcommand(
                SubCommand::with_name("prune")
                    .about("Prunes duplicate packages from the repository"),
            )
            .subcommand(SubCommand::with_name("init").about(
                "Clones all git repositories from mlc.toml branching from current directory",
            ))
            .subcommand(
                SubCommand::with_name("pull").alias("update").about(
                    "Pulls all git repositories from mlc.toml branching from current directory",
                ),
            )
            .subcommand(
                SubCommand::with_name("config").about("Create and/or open local config file"),
            )
            .settings(&[
                AppSettings::GlobalVersion,
                AppSettings::VersionlessSubcommands,
                AppSettings::ArgRequiredElseHelp,
                AppSettings::InferSubcommands,
            ]);
        app
    }

    let matches = build_app().get_matches();

    if Path::exists("mlc.toml".as_ref()) && Path::exists(".git".as_ref()) {
        info(
            "In a git repository, pulling latest mlc.toml. It is advised you run mlc pull/update"
                .to_string(),
        );
        Command::new("git")
            .arg("pull")
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    }

    if Path::exists("../.git".as_ref()) {
        info("Parent directory is a git directory, pulling latest mlc.toml. It is advised you run mlc pull/update in all malachite directories".to_string());
        let dir = env::current_dir().unwrap();
        env::set_current_dir("../").unwrap();
        Command::new("git")
            .arg("pull")
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
        env::set_current_dir(dir).unwrap();
    }

    if let true = matches.is_present("init") {
        let config = workspace::read_cfg();
        if config.mode == "workspace" {
            for r in config.repo {
                info(format!("Cloning (workspace mode): {}", r));
                Command::new("git")
                    .args(&["clone", &r])
                    .spawn()
                    .unwrap()
                    .wait()
                    .unwrap();
            }
        } else if config.mode == "repository" {
            for r in config.repo {
                info(format!("Cloning (repository mode): {}", r));
                Command::new("git")
                    .args(&["clone", &r])
                    .spawn()
                    .unwrap()
                    .wait()
                    .unwrap();
            }
        } else {
            crash("Invalid mode in mlc.toml".to_string(), 1);
        }
    }

    if let true = matches.is_present("build") {
        let config = workspace::read_cfg();
        let mut packages: Vec<String> = matches
            .subcommand_matches("build")
            .unwrap()
            .values_of_lossy("package(s)")
            .unwrap_or_default();

        let exclude: Vec<String> = matches
            .subcommand_matches("build")
            .unwrap()
            .values_of_lossy("exclude")
            .unwrap_or_default();

        for pkg in &exclude {
            packages.retain(|x| &*x != pkg);
        }

        if config.mode != "repository" {
            crash("Cannot build packages in workspace mode".to_string(), 2);
        }

        let mut repos: Vec<String> = vec![];
        for r in config.repo {
            let split = r.split('/').collect::<Vec<&str>>();
            let a = split.last().unwrap();
            repos.push(a.parse().unwrap());
        }

        if matches
            .subcommand_matches("build")
            .unwrap()
            .is_present("exclude")
        {
            for ex in exclude {
                repos.retain(|x| *x != ex);
            }
        }

        for pkg in packages {
            if !repos.contains(&pkg) {
                crash(format!("Package {} not found in repos in mlc.toml", pkg), 3);
            } else {
                repository::build(pkg);
            }
        }

        if matches
            .subcommand_matches("build")
            .unwrap()
            .is_present("all")
        {
            for pkg in repos {
                repository::build(pkg);
            }
        }

        if matches
            .subcommand_matches("build")
            .unwrap()
            .is_present("regen")
        {
            repository::generate();
        }
    }

    if let true = matches.is_present("pull") {
        let config = workspace::read_cfg();
        let cdir = env::current_dir().unwrap();
        for r in config.repo {
            info(format!("Entering working directory: {}", r));
            let dir = format!(
                "{}/{}",
                env::current_dir().unwrap().display(),
                r.split('/').collect::<Vec<&str>>().last().unwrap()
            );
            env::set_current_dir(dir).unwrap();
            Command::new("git")
                .args(&["pull", &r])
                .spawn()
                .unwrap()
                .wait()
                .unwrap();
            env::set_current_dir(&cdir).unwrap();
        }
    }

    if let true = matches.is_present("repo-gen") {
        let config = read_cfg();
        if config.mode != "repository" {
            panic!("Cannot build packages in workspace mode")
        }
        info(format!("Generating repository: {}", config.name.unwrap()));
        repository::generate();
    }

    if let true = matches.is_present("prune") {
        let config = read_cfg();
        if &config.mode != "repository" {
            panic!("Cannot build packages in workspace mode")
        }
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
                .args(&["-c", &format!("ls out/{}*.tar.* -w 1 | sort -r", pkg)])
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
            info(format!(
                "Pruning duplicates: {}",
                packages_to_del.join(", ")
            ));
        }

        for pkg in packages_to_del {
            fs::remove_file(pkg).unwrap();
        }
    }

    if let true = matches.is_present("config") {
        if !Path::exists("mlc.toml".as_ref()) {
            create_config();
        }
        let editor = env::var("EDITOR").unwrap_or_else(|_| "nano".to_string());
        Command::new(editor)
            .arg("mlc.toml")
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    }
}
