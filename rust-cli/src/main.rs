#[macro_use]
extern crate log;
extern crate clap_log_flag;
extern crate clap_verbosity_flag;
extern crate structopt;

use ansi_term::Color::Red;
use exitfailure::ExitFailure;
use failure::ResultExt;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Cli {
    name: String,
    #[structopt(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
    #[structopt(flatten)]
    log: clap_log_flag::Log,
}

fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();
    args.log
        .log_all(Some(args.verbose.log_level()))
        .with_context(|_| "Could not create log writer")?;
    debug!("Starting");
    println!("Hello, {}!", Red.paint(args.name));

    error!("error level");
    warn!("warn level");
    info!("info level");
    debug!("debug level");
    trace!("trace level");

    Ok(())
}
