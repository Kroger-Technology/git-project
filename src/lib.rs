mod commands;
pub mod err;
mod explore;
pub mod options;
mod util;

use crate::commands::{check, clone, list, organize};

#[cfg(test)]
mod test;

use crate::err::Result;
use crate::options::Options;

pub fn run(opts: &Options) -> Result<()> {
    match &opts.command {
        options::Command::Clone(clone_opts) => clone::run(&clone_opts),
        options::Command::List(list_opts) => list::run(&list_opts),
        options::Command::Check(list_opts) => check::run(&list_opts),
        options::Command::Organize(organize_opts) => organize::run(&organize_opts),
    }
}
