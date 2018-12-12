use crate::util;
use rstest::*;
use url::Url;

#[rstest_parametrize(
    clone_url,
    case("git@github.com:/KrogerTechnology/git-project.git"),
    case("git@github.com:/KrogerTechnology/git-project"),
    case("foo@github.com:/KrogerTechnology/git-project.git"),
    case("bar@github.com:/KrogerTechnology/git-project")
)]
fn test_find_dir_github_ssh(clone_url: &str) {
    let dir = util::find_dir_ssh("/users/foo/base", clone_url).unwrap();
    assert_eq!(
        dir.to_string_lossy(),
        "/users/foo/base/github.com/KrogerTechnology/git-project"
    );
}

#[rstest_parametrize(
    clone_url,
    case("git@gitlab.com:/KrogerTechnology/git-project.git"),
    case("git@gitlab.com:/KrogerTechnology/git-project"),
    case("foo@gitlab.com:/KrogerTechnology/git-project.git"),
    case("bar@gitlab.com:/KrogerTechnology/git-project")
)]
fn test_find_dir_gitlab_ssh(clone_url: &str) {
    let dir = util::find_dir_ssh("/users/foo/base", clone_url).unwrap();
    assert_eq!(
        dir.to_string_lossy(),
        "/users/foo/base/gitlab.com/KrogerTechnology/git-project"
    );
}

#[rstest_parametrize(
    clone_url,
    case("git@gitlab.kroger.com:/KrogerTechnology/git-project.git"),
    case("git@gitlab.kroger.com:/KrogerTechnology/git-project"),
    case("foo@gitlab.kroger.com:/KrogerTechnology/git-project.git"),
    case("bar@gitlab.kroger.com:/KrogerTechnology/git-project")
)]
fn test_find_dir_gitlab_internal_ssh(clone_url: &str) {
    let dir = util::find_dir_ssh("/users/foo/base", clone_url).unwrap();
    assert_eq!(
        dir.to_string_lossy(),
        "/users/foo/base/gitlab.kroger.com/KrogerTechnology/git-project"
    );
}

#[rstest_parametrize(
    clone_url,
    case("https://github.com/KrogerTechnology/git-project.git"),
    case("https://github.com/KrogerTechnology/git-project"),
    case("http://github.com/KrogerTechnology/git-project.git"),
    case("http://github.com/KrogerTechnology/git-project")
)]
fn test_find_dir_github_url(clone_url: &str) {
    let dir = util::find_dir_url("/users/foo/base", &Url::parse(clone_url).unwrap()).unwrap();
    assert_eq!(
        dir.to_string_lossy(),
        "/users/foo/base/github.com/KrogerTechnology/git-project"
    );
}

#[rstest_parametrize(
    clone_url,
    case("https://gitlab.com/KrogerTechnology/git-project.git"),
    case("https://gitlab.com/KrogerTechnology/git-project"),
    case("http://gitlab.com/KrogerTechnology/git-project.git"),
    case("http://gitlab.com/KrogerTechnology/git-project")
)]
fn test_find_dir_gitlab_url(clone_url: &str) {
    let dir = util::find_dir_url("/users/foo/base", &Url::parse(clone_url).unwrap()).unwrap();
    assert_eq!(
        dir.to_string_lossy(),
        "/users/foo/base/gitlab.com/KrogerTechnology/git-project"
    );
}

#[rstest_parametrize(
    clone_url,
    case("https://gitlab.kroger.com/KrogerTechnology/git-project.git"),
    case("https://gitlab.kroger.com/KrogerTechnology/git-project"),
    case("http://gitlab.kroger.com/KrogerTechnology/git-project.git"),
    case("http://gitlab.kroger.com/KrogerTechnology/git-project")
)]
fn test_find_dir_gitlab_internal_url(clone_url: &str) {
    let dir = util::find_dir_url("/users/foo/base", &Url::parse(clone_url).unwrap()).unwrap();
    assert_eq!(
        dir.to_string_lossy(),
        "/users/foo/base/gitlab.kroger.com/KrogerTechnology/git-project"
    );
}
