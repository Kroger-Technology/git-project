use crate::{
    err::{Err, Result},
    options::CloneOptions,
    util,
};
use std::{fs, path, process};

pub fn run(clone_opts: &CloneOptions) -> Result<()> {
    let full_path = util::find_dir(&clone_opts.base.base_dir, &clone_opts.clone_url)?;

    if clone_opts.only_print_location {
        println!("{}", full_path.display());
    } else {
        clone(clone_opts, &full_path)?;

        println!("cloned to {}", full_path.display());
    }

    Ok(())
}

pub fn clone<P>(clone_opts: &CloneOptions, clone_path: P) -> Result<()>
where
    P: AsRef<path::Path>,
{
    let clone_path = clone_path.as_ref();

    if fs::read_dir(clone_path).is_ok() {
        // Do nothing if the clone directory already exists
        return Ok(());
    }

    fs::create_dir_all(clone_path.parent().unwrap()).expect("Failed to create directory");

    let status = process::Command::new("git")
        .args(&["clone", &clone_opts.clone_url, clone_path.to_str().unwrap()])
        .stdout(process::Stdio::null())
        .status()?;

    if !status.success() {
        return Err(Err::SubcommandFailed(status.code()));
    }

    Ok(())
}
