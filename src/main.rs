use git_project::{err::Err, options::Options};
use std::process;
use structopt::StructOpt;

fn main() {
    let opts = Options::from_args();

    if let Err(e) = git_project::run(&opts) {
        eprintln!("{}", e);

        if let Err::SubcommandFailed(Some(code)) = e {
            process::exit(code);
        }

        process::exit(1);
    }
}
