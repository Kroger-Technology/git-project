mod check;
mod clone;
pub mod err;
mod explore;
mod list;
pub mod options;
mod util;

#[cfg(test)]
mod test;

use crate::err::Result;
use crate::options::Options;

pub fn run(opts: &Options) -> Result<()> {
    match &opts.command {
        options::Command::Clone(clone_opts) => clone::run(&clone_opts),
        options::Command::List(list_opts) => list::run(&list_opts),
        options::Command::Check(list_opts) => check::run(&list_opts),
    }
}
