use std::path;
use structopt::StructOpt;

/// A manager for all of your git projects
#[derive(StructOpt)]
pub struct Options {
    #[structopt(flatten)]
    pub base: BaseOptions,

    #[structopt(flatten)]
    pub command: Command,
}

#[derive(StructOpt)]
pub struct BaseOptions {
    /// The base directory that all of your repositories are inside
    #[structopt(
        short = "-d",
        long = "--base-dir",
        env = "GIT_PROJECT_BASE_DIR",
        parse(from_os_str)
    )]
    pub base_dir: path::PathBuf,
}

#[derive(StructOpt)]
pub enum Command {
    /// Clone a new project onto your system
    ///
    /// If you were to clone the project https://github.com/KrogerTechnology/git-project.git,
    /// git-project would put that in the path BASE_DIR/github.com/KrogerTechnology/git-project.
    #[structopt(name = "clone")]
    Clone(CloneOptions),

    /// List all repositories under the base path
    #[structopt(name = "list")]
    List(ListOptions),

    /// Check all repositories under the base path to ensure the are up to date with remotes.
    #[structopt(name = "check")]
    Check(CheckOptions),
}

#[derive(StructOpt)]
pub struct CloneOptions {
    /// Print the clone location for the given URL and exit
    #[structopt(short = "-n", long = "--dry-run")]
    pub only_print_location: bool,

    /// The URL of the project to be cloned. Can be URL or ssh path.
    #[structopt(name = "URL")]
    pub clone_url: String,
}

#[derive(StructOpt)]
pub struct ListOptions {
    /// Do not stop recursing when a .git folder is found
    #[structopt(short = "-r", long = "--deep-recurse")]
    pub deep_recurse: bool,
}

#[derive(StructOpt)]
pub struct CheckOptions {
    #[structopt(flatten)]
    pub list_opts: ListOptions,

    /// Print a summary of the repositories
    #[structopt(short = "-s", long = "--summarize")]
    pub summarize: bool,
}
