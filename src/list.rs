use crate::{
    err::Result,
    explore,
    options::{BaseOptions, ListOptions},
};

use std::io::{self, prelude::*};

pub fn run(base_opts: &BaseOptions, list_opts: &ListOptions) -> Result<()> {
    let dirs = explore::find_git_folders(&base_opts.base_dir, list_opts.deep_recurse)?;
    let stdin = io::stdout();
    let mut lock = stdin.lock();

    for d in dirs {
        writeln!(lock, "{}", d.display())?;
    }

    Ok(())
}
