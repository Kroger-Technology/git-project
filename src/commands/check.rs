use crate::{err::Result, explore, options::CheckOptions};
use rayon::prelude::*;
use std::{collections::HashMap, fmt, iter, path};

struct Repository {
    path: String,
    warnings: Vec<Warning>,
}

enum Warning {
    NoRemotes,
    DirtyWorkingDir,
    LocalCommitsNotOnRemote {
        remote: String,
        branch: String,
        ahead_by: usize,
    },
    LocalBranchNotOnRemote {
        remote: String,
        branch: String,
    },
}

#[derive(Default)]
struct Statistics {
    warnings: usize,
    total_repos: usize,
    repos_with_warnings: usize,
    repos_no_warnings: usize,
}

impl fmt::Display for Repository {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.path)?;

        for warning in &self.warnings {
            writeln!(f, "  - {}", warning)?;
        }

        Ok(())
    }
}

impl fmt::Display for Warning {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Warning::NoRemotes => write!(f, "no remotes configured"),
            Warning::DirtyWorkingDir => write!(f, "working directory has changes not checked in"),
            Warning::LocalCommitsNotOnRemote {
                remote,
                branch,
                ahead_by,
            } => write!(
                f,
                "local branch {0} ahead of {1}/{0} by {2} commits",
                branch, remote, ahead_by
            ),
            Warning::LocalBranchNotOnRemote { remote, branch } => write!(
                f,
                "local branch {} does not exist on remote {}",
                branch, remote
            ),
        }
    }
}

impl Repository {
    fn get_stats(&self) -> Statistics {
        let empty = self.warnings.is_empty();
        let repos_with_warnings = if empty { 0 } else { 1 };
        let repos_no_warnings = if empty { 1 } else { 0 };

        Statistics {
            warnings: self.warnings.len(),
            total_repos: 1,
            repos_with_warnings,
            repos_no_warnings,
        }
    }
}

impl iter::Sum<Statistics> for Statistics {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        let mut stats: Statistics = Default::default();

        for s in iter {
            stats.warnings += s.warnings;
            stats.total_repos += s.total_repos;
            stats.repos_with_warnings += s.repos_with_warnings;
            stats.repos_no_warnings += s.repos_no_warnings;
        }

        stats
    }
}

impl fmt::Display for Statistics {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "---- Summary ---")?;
        writeln!(f, "Warnings: {}", self.warnings)?;
        writeln!(f, "Scanned repositories: {}", self.total_repos)?;
        writeln!(
            f,
            "Repositories with warnings: {}",
            self.repos_with_warnings
        )?;
        writeln!(
            f,
            "Repositories with no warnings: {}",
            self.repos_no_warnings
        )
    }
}

pub fn run(check_opts: &CheckOptions) -> Result<()> {
    let paths = explore::find_git_folders(&check_opts.base.base_dir, check_opts.list.deep_recurse)?;

    let stats: Statistics = paths
        .par_iter()
        .map(check_git_dir_entry)
        .filter_map(|result| match result {
            Ok(x) => Some(x),
            Err(e) => {
                eprintln!("Error received: {}", e);
                None
            }
        })
        .map(|repo| {
            if !repo.warnings.is_empty() {
                println!("{}", repo);
            }

            repo.get_stats()
        })
        .sum();

    if check_opts.summarize {
        println!("{}", stats);
    }

    Ok(())
}

fn check_git_dir_entry(git_path: &path::PathBuf) -> Result<Repository> {
    let repo = git2::Repository::open(&git_path)?;

    let mut warnings = Vec::new();

    if !is_clean(&repo)? {
        warnings.push(Warning::DirtyWorkingDir);
    }

    let remote_branches = repo.branches(Some(git2::BranchType::Remote))?;
    let local_branches = repo.branches(Some(git2::BranchType::Local))?;

    let remotes = generate_remote_tips(strip_branch_errors(remote_branches))?;
    let local_tips = generate_tips(strip_branch_errors(local_branches))?;

    if remotes.is_empty() {
        warnings.push(Warning::NoRemotes);
    }

    for (local_branch, local_sha) in &local_tips {
        for (remote_name, remote_tips) in &remotes {
            match remote_tips.get(local_branch) {
                Some(remote_sha) => {
                    if remote_sha != local_sha {
                        let (ahead_by, _) = repo.graph_ahead_behind(*local_sha, *remote_sha)?;

                        if ahead_by > 0 {
                            warnings.push(Warning::LocalCommitsNotOnRemote {
                                remote: remote_name.clone(),
                                branch: local_branch.clone(),
                                ahead_by,
                            });
                        }
                    }
                }
                None => {
                    warnings.push(Warning::LocalBranchNotOnRemote {
                        remote: remote_name.clone(),
                        branch: local_branch.clone(),
                    });
                }
            }
        }
    }

    Ok(Repository {
        path: format!("{}", git_path.display()),
        warnings,
    })
}

fn is_clean(repo: &git2::Repository) -> Result<bool> {
    let statuses = repo.statuses(Some(git2::StatusOptions::new().include_untracked(true)))?;

    for status in statuses.iter() {
        if status.status() != git2::Status::CURRENT {
            return Ok(false);
        }
    }

    Ok(true)
}

fn strip_branch_errors(branches: git2::Branches) -> impl Iterator<Item = git2::Branch> {
    branches.filter_map(|x| x.map(|(branch, _)| branch).ok())
}

fn generate_remote_tips<'a, I>(branches: I) -> Result<HashMap<String, HashMap<String, git2::Oid>>>
where
    I: Iterator<Item = git2::Branch<'a>>,
{
    let mut map = HashMap::new();

    for branch in branches {
        let name = match branch.name() {
            Ok(Some(n)) => n,
            _ => break,
        };

        let mut name_parts = name.split('/');
        let remote_name = name_parts.next().unwrap();

        let (ref_name, commit) = tip(&branch)?;

        let mut ref_name_parts = ref_name.split('/');
        ref_name_parts.next().unwrap();

        let ref_name = itertools::join(ref_name_parts, "/");

        let branch_items = map
            .entry(remote_name.to_owned())
            .or_insert_with(HashMap::new);
        branch_items.insert(ref_name, commit);
    }

    Ok(map)
}

fn generate_tips<'a, I>(branches: I) -> Result<HashMap<String, git2::Oid>>
where
    I: Iterator<Item = git2::Branch<'a>>,
{
    let mut map = HashMap::new();

    for branch in branches {
        let (ref_name, commit) = tip(&branch)?;
        map.insert(ref_name, commit);
    }

    Ok(map)
}

fn tip<'a>(branch: &git2::Branch<'a>) -> Result<(String, git2::Oid)> {
    let branch_ref = branch.get();

    let ref_name = branch_ref.shorthand().unwrap();
    let ref_commit = branch_ref.peel_to_commit()?;

    Ok((ref_name.to_owned(), ref_commit.id()))
}
