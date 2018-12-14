use crate::{err::Result, explore, options::OrganizeOptions, util};
use std::fs;

pub fn run(opts: &OrganizeOptions) -> Result<()> {
    let dirs = explore::find_git_folders(&opts.dir, false)?;

    let mut count = 0;

    for dir in dirs {
        let git_path = dir.join(".git");

        let repo = git2::Repository::open(git_path)?;

        let new_dir = match canonical_url(&repo)? {
            Some(remote_url) => util::find_dir(&opts.new_dir, &remote_url)?,
            None => "no-remote".into(),
        };

        println!("{} -> {}", dir.display(), new_dir.display());

        if !opts.dry_run {
            fs::create_dir_all(new_dir.parent().unwrap())?;

            fs::rename(dir, new_dir)?;
        }

        count += 1;
    }

    if opts.dry_run {
        println!("Will move {} directories", count);
    } else {
        println!("Moved {} directories", count);
    }

    Ok(())
}

fn canonical_url(repo: &git2::Repository) -> Result<Option<String>> {
    if let Ok(remote) = repo.find_remote("origin") {
        if let Some(url) = remote.url() {
            return Ok(Some(url.into()));
        }
    }

    first_remote_with_url(repo)
}

fn first_remote_with_url(repo: &git2::Repository) -> Result<Option<String>> {
    let remotes = repo.remotes()?;

    for remote_name in remotes.iter() {
        if let Some(remote_name) = remote_name {
            if let Ok(remote) = repo.find_remote(remote_name) {
                if let Some(url) = remote.url() {
                    return Ok(Some(url.into()));
                }
            }
        }
    }

    Ok(None)
}
