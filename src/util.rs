use crate::err::{Err, Result};
use lazy_static::lazy_static;
use regex::Regex;
use std::path;

lazy_static! {
    static ref SSH_REGEX: Regex = Regex::new(r"[^@]+@([^:]+):(.*)").unwrap();
}

pub fn find_dir<P>(base_dir: P, clone_url: &str) -> Result<path::PathBuf>
where
    P: AsRef<path::Path>,
{
    match url::Url::parse(clone_url) {
        Ok(u) => find_dir_url(base_dir, &u),
        Err(_) => find_dir_ssh(base_dir, &clone_url),
    }
}

pub fn find_dir_ssh<P>(base_dir: P, clone_url: &str) -> Result<path::PathBuf>
where
    P: AsRef<path::Path>,
{
    let group = match SSH_REGEX.captures(clone_url) {
        Some(g) => g,
        None => return Err(Err::InvalidUrl),
    };

    let mut full_path = base_dir.as_ref().to_path_buf();

    full_path = full_path.join(&group[1]);
    for segment in remove_dotgit(&group[2]).split('/') {
        full_path.push(segment);
    }

    Ok(full_path)
}

pub fn find_dir_url<P>(base_dir: P, parsed_url: &url::Url) -> Result<path::PathBuf>
where
    P: AsRef<path::Path>,
{
    let host = match parsed_url.host() {
        Some(url::Host::Domain(d)) => d,
        Some(_) => return Err(Err::NonDomainHost),
        None => return Err(Err::NoHost),
    };

    let mut full_path = base_dir.as_ref().join(host);

    if let Some(segments) = parsed_url.path_segments() {
        for segment in segments {
            full_path.push(remove_dotgit(segment));
        }
    }

    Ok(full_path)
}

fn remove_dotgit(s: &str) -> &str {
    if s.ends_with(".git") {
        s.split_at(s.len() - 4).0
    } else {
        s
    }
}

pub trait PathRelativizeExtension {
    fn relative_to(self, base_dir: &path::Path) -> Option<path::PathBuf>;
    fn normalize_relative_to(self, base_dir: &path::Path) -> path::PathBuf;
}

impl PathRelativizeExtension for &path::Path {
    fn relative_to(self, base_dir: &path::Path) -> Option<path::PathBuf> {
        pathdiff::diff_paths(&self, base_dir)
    }

    fn normalize_relative_to(self, base_dir: &path::Path) -> path::PathBuf {
        self.relative_to(base_dir).unwrap_or_else(|| self.into())
    }
}
