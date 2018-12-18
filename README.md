# `git-project`

`git-project` allows you to quickly and easily manage the large number of git
projects that can accrue on a modern developer's computer.

## Installing

```
$ cargo install git-project
```

# Usage

```
λ git project -h
git-project 0.1.0
Nate Mara <nate.mara@kroger.com>
A manager for all of your git projects

USAGE:
    git-project <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    check              Check all repositories under the base path to ensure the are up to date with remotes
    clone              Clone a new project into a folder based on the remote URL
    gen-completions    Generate CLI completions for your shell of choice
    help               Prints this message or the help of the given subcommand(s)
    list               List all repositories under the base path
    organize           Organize an existing directory of git repositories into a normalized format based on remotes
```

# Base Directory

All of the `git-project` subcommands accept a `--base-dir` or `-d` argument,
or a `GIT_PROJECT_BASE_DIR` environment variable pointing to a directory
where all of your git repositories are located.

# Subcommands

`git-project` has several subcommands that each deal with a specific aspect
of managing git repositories.

- clone
- check
- list
- organize
- gen-completions

## `git project organize OLD_DIR NEW_DIR`

This subcommand is used to organize a directory containing git repositories.
Each repository will be placed into a folder based on its remote path. By
default the "origin" remote is used, but if there is no origin, the first
remote with a configured URL will be used. Here is a small example of how it
can be used:

```bash
$ cd unorganized/foo
$ git init
$ git remote add origin https://github.com/Kroger-Technology/git-project.git

$ cd ../bar
$ git init
$ git remote add origin git@gitlab.internal.com/you/git-project.git

$ cd ../..
$ git project organize unorganized organized

$ git project list -d organized
YOUR_DIR/organized/github.com/Kroger-Technology/git-project
YOUR_DIR/organized/gitlab.internal.com/you/git-project
```

### `--dry-run`

Do not move any files, just print out the moves that this command will
execute.

## `git project clone URL`

This subcommand is used to clone a repository onto your computer. What's the
difference between this, and a normal `git clone`? `git project clone` will
automatically clone the project to a folder matching the URL that you give it!

- `https://github.com/Kroger-Technology/git-project.git` is cloned to
  `/base-dir/github.com/Kroger-Technology/git-project`

- `git@gitlab.internal.com:you/git-project.git` is cloned to
  `/base-dir/gitlab.internal.com/you/git-project`

Having your repositories in this format is not required for `git-project`'s
other subcommands to work.

## `git project check`

This subcommand allows you to check the status of all git repositories under
your base directory. This can be used to ensure that all changes on your
computer are on a remote somewhere so that your work will not be lost if
something happens to your computer.

```
λ git-status-recursive
/Users/nate/projects/not-mine/rust-peg

- branch left-recursion does not exist on remote origin
- branch template-return does not exist on remote origin

/Users/nate/projects/personal/git-project

- local changes not checked in
- local branch master ahead of origin/master by 4 commits

/Users/nate/projects/personal/rbr-finding-exoplanets/cargo_home/registry/index/github.com-1ecc6299db9ec823

- local changes not checked in

/Users/nate/projects/not-mine/DefinitelyTyped

- branch master does not exist on remote fork

---- Summary ---
Warnings: 6
Scanned repositories: 74
Repositories with warnings: 4
Repositories with no warnings: 70
```

### `--deep-recurse`

The `--deep-recurse` flag works exactly the same for this command as with the
[`list`](#git-project-list) subcommand, but instead of just listing
submodules, this command will check them.

### `--summarize`

This flag prints a summary of all the repositories scanned.

### Warnings reported

- Working directory changes not checked in to index
- No remotes configured
- Local branch has commits that remote does not have
- Local branch does not exist on remote

## `git project list`

This subcommand recurses the directory tree under your base directory and
prints out every git repository it finds.

NOTE: by default, when encountering a `.git` folder, this search will stop
recursing as an optimization. To search for submodules, use the
`--deep-recurse` flag.

Here is an example directory tree:

```
foo/
foo/.git
foo/bar/src/main.rs
foo/bar/Cargo.toml
foo/bar/.git
bar/.git
baz/.git
```

And some example outputs

```
$ git project list
/home/you/projects/foo
/home/you/projects/bar
/home/you/projects/baz

$ git project list --deep-recurse
/home/you/projects/foo
/home/you/projects/foo/bar
/home/you/projects/bar
/home/you/projects/baz
```

## `git project gen-completions SHELL`

This subcommand will generate autocomplete scripts for your shell of choice.
Scripts will be printed to stdout. Current shell options are:

- bash
- zsh
- fish
- powershell
- elvish
