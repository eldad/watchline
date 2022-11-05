use std::{
    fmt::Debug,
    time::Duration, process::Command,
};
use std::io::Write;
use clap::Parser;
use eyre::WrapErr;

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

    /// Continue on error (if commands exits with code other than 0).
    /// Use with care - this is essentially an infinite loop.
    /// The command will keep on running until a SIGTERM is received (e.g., via CTRL-C).
    #[clap(short, long)]
    continue_on_error: bool,

    /// Command
    #[clap()]
    cmd: String,

    /// Arguments
    #[clap()]
    args: Vec<String>,
}

fn main() -> simple_eyre::Result<()> {
    simple_eyre::install()?;

    let args = Args::parse();

    let interval = args.interval;
    let continue_on_error = args.continue_on_error;

    let mut watched_cmd = Command::new(&args.cmd);
    watched_cmd.args(&args.args);

    loop {
        let output = watched_cmd.output().wrap_err_with(|| "Cannot execute command")?;

        std::io::stdout().write_all(&output.stdout).wrap_err_with(|| "Cannot write stdout")?;
        std::io::stderr().write_all(&output.stderr).wrap_err_with(|| "Cannot write stderr")?;

        if !continue_on_error && !output.status.success() {
            std::process::exit(output.status.code().ok_or_else(|| eyre::eyre!("no exit code"))?);
        }

        std::thread::sleep(Duration::from_secs_f64(interval));
    }
}
