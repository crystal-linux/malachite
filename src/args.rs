use clap::{ArgAction, Parser, Subcommand};

#[derive(Debug, Clone, Parser)]
#[clap(name = "Malachite", version = env ! ("CARGO_PKG_VERSION"), about = env ! ("CARGO_PKG_DESCRIPTION"))]
pub struct Args {
    #[clap(subcommand)]
    pub subcommand: Option<Operation>,

    /// Sets the level of verbosity
    #[clap(long, short, global(true), action = ArgAction::SetTrue)]
    pub verbose: bool,

    /// Complete operations without prompting user
    #[clap(long = "noconfirm", global(true), action = ArgAction::SetTrue)]
    pub no_confirm: bool,

    /// Excludes packages from given operation, if applicable
    #[clap(short = 'x', long = "exclude", action = ArgAction::Append, takes_value = true)]
    pub exclude: Vec<String>,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Operation {
    /// Builds the given packages
    #[clap(name = "build", aliases = & ["b"])]
    Build {
        /// The packages to operate on
        #[clap(name = "package(s)", action = ArgAction::Append, index = 1)]
        packages: Vec<String>,

        /// Does not regenerate repository after building given package(s)
        #[clap(short = 'n', long = "no-regen", action = ArgAction::SetTrue)]
        no_regen: bool,
    },

    /// Generates Pacman repository from built packages
    #[clap(name = "repo-gen", aliases = & ["repo", "r"])]
    RepoGen,

    /// Clones all git repositories from mlc.toml branching from current directory
    #[clap(name = "clone", aliases = & ["init", "i", "c"])]
    Clone,

    /// Removes everything in directory except for mlc.toml
    #[clap(name = "clean", aliases = & ["clean", "cl", "reset"])]
    Clean,

    /// Pulls in git repositories from mlc.toml branching from current directory
    #[clap(name = "pull", aliases = & ["u"])]
    Pull {
        /// The packages to operate on
        #[clap(name = "package(s)", help = "The packages to operate on", action = ArgAction::Append, index = 1)]
        packages: Vec<String>,
    },

    /// Create and/or open local config file
    #[clap(name = "config", aliases = & ["conf"])]
    Config,
}
