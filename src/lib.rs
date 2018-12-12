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
        options::Command::Clone(clone_opts) => clone::run(&opts.base, &clone_opts),
        options::Command::List(list_opts) => list::run(&opts.base, &list_opts),
        options::Command::Check(list_opts) => check::run(&opts.base, &list_opts),
    }
}
