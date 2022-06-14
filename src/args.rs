use clap::{ArgAction, Parser, Subcommand};

#[derive(Debug, Clone, Parser)]
#[clap(name="Malachite", version=env!("CARGO_PKG_VERSION"), about=env!("CARGO_PKG_DESCRIPTION"))]
pub struct Args {
    #[clap(subcommand)]
    pub subcommand: Option<Operation>,

    /// Sets the level of verbosity
    #[clap(long, short, global(true), action=ArgAction::Count)]
    pub verbose: u8,

    /// Complete operations without prompting user
    #[clap(long="noconfirm", global(true), action=ArgAction::SetTrue)]
    pub no_confirm: bool,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Operation {
    /// Builds the given packages
    #[clap(name="build", aliases=&["b"])]
    Build {
        /// The packages to operate on
        #[clap(name="package(s)", action=ArgAction::Append, index=1)]
        packages: Vec<String>,

        /// Builds all packages in mlc.toml (except if -x is specified)
        #[clap(long="all", takes_value=false, action=ArgAction::SetTrue, conflicts_with="package(s)")]
        all: bool,

        /// Excludes packages from given operation
        #[clap(short='x', long="exclude", action=ArgAction::Append, takes_value=true)]
        exclude: Vec<String>,

        /// Does not regenerate repository after building given package(s)
        #[clap(short='n', long="no-regen", action=ArgAction::SetTrue)]
        no_regen: bool,
    },

    /// Generates repository from built packages
    #[clap(name="repo-gen", aliases=&["r"])]
    RepoGen,

    /// Prunes duplicate packages from the repository
    #[clap(name="prune", aliases=&["p"])]
    Prune,

    /// Clones all git repositories from mlc.toml branching from current directory
    #[clap(name="init", aliases=&["i"])]
    Init,

    /// Pulls in git repositories from mlc.toml branching from current directory
    #[clap(name="pull", aliases=&["u"])]
    Pull {
        /// The packages to operate on
        #[clap(name="package(s)", help="The packages to operate on", action=ArgAction::Append, index=1)]
        packages: Vec<String>,

        /// Pulls from all git repositories from mlc.toml branching from current directory
        #[clap(long="all", action=ArgAction::SetTrue, conflicts_with="package(s)")]
        all: bool,
    },

    /// Create and/or open local config file
    #[clap(name="config", aliases=&["c"])]
    Config,
}
