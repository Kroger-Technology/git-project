use crate::err::Result;
use std::path;
use walkdir;

const MAX_DEPTH: usize = 100;

pub fn find_git_folders<P>(base_path: P, deep_recurse: bool) -> Result<Vec<path::PathBuf>>
where
    P: AsRef<path::Path>,
{
    let mut paths = Vec::new();
    collect_git_folders(base_path, "", 0, &mut paths, !deep_recurse)?;

    Ok(paths)
}

fn collect_git_folders<P, R>(
    base_path: P,
    child_path: R,
    depth: usize,
    paths: &mut Vec<path::PathBuf>,
    shallow_recurse: bool,
) -> Result<()>
where
    P: AsRef<path::Path>,
    R: AsRef<path::Path>,
{
    let full_path = base_path.as_ref().join(child_path).canonicalize()?;

    for entry_res in walkdir::WalkDir::new(&full_path).min_depth(1).max_depth(1) {
        let entry = entry_res?;
        let entry_path = entry.path();

        if entry_path.is_dir() {
            if entry.file_name() == ".git" {
                paths.push(full_path.clone());
                if shallow_recurse {
                    return Ok(());
                }
            } else if depth < MAX_DEPTH {
                collect_git_folders(&full_path, entry_path, depth + 1, paths, shallow_recurse)?;
            }
        }
    }

    Ok(())
}
