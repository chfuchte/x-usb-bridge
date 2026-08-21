use std::time::Duration;

use anyhow::Result;
use clap::{ArgAction, arg, command, value_parser};

pub(crate) enum Mode {
    ShowLicense,
    ShowVersion,
    Serve(ServeArgs),
    Pipe(PipeArgs),
}

pub(crate) struct ServeArgs {
    port: u16,
}

impl ServeArgs {
    pub(crate) fn port(&self) -> u16 {
        self.port
    }
}

pub(crate) struct PipeArgs {
    timeout: Option<Duration>,
}

impl PipeArgs {
    pub(crate) fn timeout(&self) -> Option<Duration> {
        self.timeout
    }
}

pub(crate) fn parse() -> Result<Mode> {
    let mut cmd = command!()
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .next_line_help(false)
        .disable_version_flag(true)
        .color(clap::ColorChoice::Never)
        .arg(
            arg!(-v --version "Print version")
                .value_parser(value_parser!(bool))
                .action(ArgAction::SetTrue),
        )
        .arg(
            arg!(--license "Print license")
                .value_parser(value_parser!(bool))
                .action(ArgAction::SetTrue),
        )
        .subcommand(
            command!("serve")
                .about("Serve a web client for remote monitoring and live audio streaming")
                .arg(
                    arg!(-p --port <PORT> "Port to listen on")
                        .value_parser(value_parser!(u16))
                        .action(ArgAction::Set)
                        .default_value("3000"),
                ),
        )
        .subcommand(
            command!("pipe")
                .about("Pipe the audio stream to stdout for further processing")
                .arg(
                    arg!(-t --timeout <TIMEOUT> "Timeout for the process to stop in milliseconds")
                        .value_parser(value_parser!(u64))
                        .action(ArgAction::Set)
                        .default_value(None),
                ),
        );

    let matches = cmd.clone().get_matches();

    if matches.get_flag("version") {
        return Ok(Mode::ShowVersion);
    }

    if matches.get_flag("license") {
        return Ok(Mode::ShowLicense);
    }

    match matches.subcommand() {
        Some(("serve", sub_matches)) => {
            let port = *sub_matches
                .get_one::<u16>("port")
                .expect("port is required and has a default value");

            let args = ServeArgs { port };

            Ok(Mode::Serve(args))
        }
        Some(("pipe", sub_matches)) => {
            let timeout = sub_matches
                .get_one::<u64>("timeout")
                .copied()
                .filter(|t| *t != 0)
                .map(Duration::from_millis);

            let args = PipeArgs { timeout };

            Ok(Mode::Pipe(args))
        }
        _ => {
            cmd.print_help()?;
            Err(anyhow::anyhow!("No subcommand provided"))
        }
    }
}
