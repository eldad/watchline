use std::{
    fmt::Debug,
    time::Duration, process::Command,
};
use std::io::Write;
use clap::Parser;

/// Watchline
///
/// Runs a command at given an interval. It is similar to `watch`, but does not clear
/// the screen.
#[derive(Parser, Debug)]
#[command(author, version, about, help_template = "
{before-help}{name} {version}
{author-with-newline}{about-with-newline}
{usage-heading} {usage}

{all-args}{after-help}")]

struct Args {
    /// Duration in seconds
    #[clap(short, long, default_value = "1.0")]
    interval: f64,

    /// Continue on error (if commands exits with code other than 0)
    #[clap(short, long)]
    continue_on_error: bool,

    /// Command
    #[clap()]
    cmd: String,

    /// Arguments
    #[clap()]
    args: Vec<String>,
}


fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let interval = args.interval;
    let continue_on_error = args.continue_on_error;

    let mut watched_cmd = Command::new(&args.cmd);
    watched_cmd.args(&args.args);

    loop {
        let output = watched_cmd.output().expect("failed to execute cmd");
        std::io::stdout().write_all(&output.stdout).unwrap();
        std::io::stderr().write_all(&output.stderr).unwrap();

        if !continue_on_error && !output.status.success() {
            std::process::exit(output.status.code().unwrap_or(0));
        }

        std::thread::sleep(Duration::from_secs_f64(interval));
    }
}
