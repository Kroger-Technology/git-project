use std::path;
use structopt::StructOpt;

/// A manager for all of your git projects
#[derive(StructOpt)]
pub struct Options {
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
    /// Clone a new project into a folder based on the remote URL
    #[structopt(name = "clone")]
    Clone(CloneOptions),

    /// List all repositories under the base path
    #[structopt(name = "list")]
    List(ListOptions),

    /// Check all repositories under the base path to ensure the are up to date with remotes
    #[structopt(name = "check")]
    Check(CheckOptions),

    /// Organize an existing directory of git repositories into a normalized format based on remotes
    #[structopt(name = "organize")]
    Organize(OrganizeOptions),
}

#[derive(StructOpt)]
pub struct CloneOptions {
    #[structopt(flatten)]
    pub base: BaseOptions,

    /// Print the clone location for the given URL and exit
    #[structopt(short = "-n", long = "--dry-run")]
    pub only_print_location: bool,

    /// The URL of the project to be cloned. Can be URL or ssh path
    #[structopt(name = "URL")]
    pub clone_url: String,
}

#[derive(StructOpt)]
pub struct ListOptions {
    #[structopt(flatten)]
    pub base: BaseOptions,

    #[structopt(flatten)]
    pub list: BaseListOptions,
}

#[derive(StructOpt)]
pub struct BaseListOptions {
    /// Do not stop recursing when a .git folder is found
    #[structopt(short = "-r", long = "--deep-recurse")]
    pub deep_recurse: bool,
}

#[derive(StructOpt)]
pub struct CheckOptions {
    #[structopt(flatten)]
    pub list: BaseListOptions,

    #[structopt(flatten)]
    pub base: BaseOptions,

    /// Print a summary of the repositories
    #[structopt(short = "-s", long = "--summarize")]
    pub summarize: bool,
}

#[derive(StructOpt)]
pub struct OrganizeOptions {
    /// Directory to organize
    #[structopt(name = "DIR", parse(from_os_str))]
    pub dir: path::PathBuf,

    /// Directory to place organized repositories in
    #[structopt(name = "NEW_DIR", parse(from_os_str))]
    pub new_dir: path::PathBuf,

    /// Print out the folders that will be moved without actually moving anything
    #[structopt(short = "-n", long = "--dry-run")]
    pub dry_run: bool,
}
