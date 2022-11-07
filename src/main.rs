/*
 * MIT License
 *
 * Copyright (c) 2022 Eldad Zack
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 */

use clap::Parser;
use eyre::WrapErr;
use std::io::Write;
use std::time::Instant;
use std::{fmt::Debug, process::Command, time::Duration};

/// Watchline
///
/// Runs a command at given an interval. It is similar to `watch`, but does not clear
/// the screen.
#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    help_template = "
{before-help}{name} {version}
{author-with-newline}{about-with-newline}
{usage-heading} {usage}

{all-args}{after-help}"
)]

struct Args {
    /// Duration in seconds
    #[clap(short, long, default_value = "1.0")]
    interval: f64,

    /// Continue on error (if commands exits with code other than 0).
    /// Use with care - this is essentially an infinite loop.
    /// The command will keep on running until a SIGTERM is received (e.g., via CTRL-C).
    #[clap(short, long)]
    continue_on_error: bool,

    /// Run command in `exec` mode. By default, the command is ran using `sh -c` to enable piping in the shell.
    #[clap(short = 'x', long)]
    exec: bool,

    /// Alternative interpreter. By default `sh` is used. Ignored when `exec` mode is used. Interpreter must accept `-c` to run a command.
    #[clap(short = 's', long, default_value = "sh")]
    interpreter: String,

    /// Precise mode. Account for run time of the command and attempt to start at exact interval.
    /// If the command execution took longer than a single interval, do not wait.
    #[clap(short = 'p', long)]
    precise: bool,

    /// Command
    ///
    /// To avoid quoting issues either use `exec` mode (-x) or provide the command quoted (passed as the first parameter to `sh -c`).
    /// When the command is quoted, you may use pipe redirection.
    #[clap()]
    cmd: String,

    /// Arguments
    ///
    /// Note that when not using `exec` mode, the arguments are not quoted.
    /// To avoid quoting issues, use `exec` mode or use a single quoted command parameter.
    #[clap()]
    args: Vec<String>,
}

fn main() -> simple_eyre::Result<()> {
    simple_eyre::install()?;

    let args = Args::parse();

    let interval = args.interval;
    let continue_on_error = args.continue_on_error;

    let mut watched_cmd = if args.exec {
        let mut cmd = Command::new(&args.cmd);
        cmd.args(&args.args);
        cmd
    } else {
        // Pass command through `sh -c`.

        let mut cmd = Command::new(args.interpreter);
        let maincmd = args.cmd;

        let fullcmd = if args.args.is_empty() {
            maincmd
        } else {
            format!("{maincmd} {}", args.args.join(" "))
        };

        let cmd_args = vec!["-c", &fullcmd];
        cmd.args(cmd_args);
        cmd
    };

    let mut start = Instant::now();

    loop {
        let output = watched_cmd.output().wrap_err_with(|| "Cannot execute command")?;

        std::io::stdout()
            .write_all(&output.stdout)
            .wrap_err_with(|| "Cannot write stdout")?;
        std::io::stderr()
            .write_all(&output.stderr)
            .wrap_err_with(|| "Cannot write stderr")?;

        if !continue_on_error && !output.status.success() {
            std::process::exit(output.status.code().ok_or_else(|| eyre::eyre!("no exit code"))?);
        }

        if args.precise {
            let time_delta = args.interval - start.elapsed().as_secs_f64();
            if time_delta > 0.0 {
                std::thread::sleep(Duration::from_secs_f64(time_delta));
                start += Duration::from_secs_f64(interval);
            } else {
                std::thread::yield_now();
                start = Instant::now();
            }
        } else {
            std::thread::sleep(Duration::from_secs_f64(interval));
        }
    }
}
