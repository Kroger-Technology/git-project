use crate::{err::Result, options};
use structopt::StructOpt;

pub fn run(opts: &options::CompletionOptions) -> Result<()> {
    options::Options::clap().gen_completions_to(
        "git-project",
        opts.shell.into(),
        &mut std::io::stdout(),
    );

    Ok(())
}
