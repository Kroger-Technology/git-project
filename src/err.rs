use auto_from::auto_from;
use std::{fmt, io};

pub type Result<T> = std::result::Result<T, Err>;

#[auto_from]
#[derive(Debug)]
pub enum Err {
    InvalidUrl,
    NonDomainHost,
    NoHost,
    Io(io::Error),
    Git2(git2::Error),
    Walkdir(walkdir::Error),
    SubcommandFailed(Option<i32>),
}

impl fmt::Display for Err {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Err::InvalidUrl => "Invalid URL".into(),
            Err::NoHost => "URL is required to have a host".into(),
            Err::NonDomainHost => "URL host is required to be a domain name".into(),
            Err::Io(e) => format!("I/O Error: {}", e),
            Err::Git2(e) => format!("Git library Error: {}", e),
            Err::Walkdir(e) => format!("Error walking directory tree: {}", e),
            Err::SubcommandFailed(Some(code)) => {
                format!("Subcommand failed with exit code {}", code)
            }
            Err::SubcommandFailed(None) => "Subcommand failed".into(),
        };

        write!(f, "{}", s)
    }
}
