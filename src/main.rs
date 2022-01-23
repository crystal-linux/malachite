use std::env;
use std::path::Path;
use std::process::Command;

use crate::internal::{crash, info};
use clap::{App, AppSettings, Arg, ArgSettings, SubCommand};
use crate::repository::create_config;

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
                            .conflicts_with("package(s)")
                    )
                    .arg(
                        Arg::with_name("exclude")
                            .short("x")
                            .long("exclude")
                            .multiple(true)
                            .takes_value(true)
                            .help("Excludes packages from given operation")
                    )
            )
            .subcommand(
                SubCommand::with_name("repo-gen").about("Generates repository from built packages"),
            )
            .subcommand(
                SubCommand::with_name("prune")
                    .about("Prunes duplicate packages older than X days from the repository")
                    .arg(
                        Arg::with_name("days")
                            .help("How old a duplicate package needs to be (in days) to be pruned")
                            .required(true)
                            .index(1),
                    ),
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
                    .args(&["clone", "--no-checkout", &r])
                    .spawn()
                    .unwrap()
                    .wait()
                    .unwrap();

                info(format!("Entering working directory: {}", r));
                let cdir = env::current_dir().unwrap();
                let dir = format!(
                    "{}/{}",
                    env::current_dir().unwrap().display(),
                    r.split('/').collect::<Vec<&str>>().last().unwrap()
                );
                env::set_current_dir(dir).unwrap();

                info(format!("Resetting unstaged files: {}", r));
                Command::new("git")
                    .arg("reset")
                    .spawn()
                    .unwrap()
                    .wait()
                    .unwrap();

                info(format!("Checking out PKGBUILD: {}", r));
                Command::new("git")
                    .args(&["checkout", "HEAD", "PKGBUILD"])
                    .spawn()
                    .unwrap()
                    .wait()
                    .unwrap();

                info(format!("Exiting work directory: {}", r));
                env::set_current_dir(cdir).unwrap();
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
            .unwrap_or(vec![]);

        let exclude: Vec<String> = matches
            .subcommand_matches("build")
            .unwrap()
            .values_of_lossy("exclude")
            .unwrap_or(vec![]);

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

        if matches.subcommand_matches("build").unwrap().is_present("exclude") {
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

        if matches.subcommand_matches("build").unwrap().is_present("all") {
            for pkg in repos {
                repository::build(pkg);
            }
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
        repository::generate();
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