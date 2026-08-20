use anyhow::Result;
use clap::{ArgAction, arg, command, value_parser};

pub(crate) enum Mode {
    ShowLicense,
    ShowVersion,
    Run(Args),
}

pub(crate) struct Args {}

impl Args {}

pub(crate) fn parse() -> Result<Mode> {
    let matches = command!()
        .next_line_help(true)
        .disable_version_flag(true)
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
        .get_matches();

    if matches.get_flag("version") {
        return Ok(Mode::ShowVersion);
    }

    if matches.get_flag("license") {
        return Ok(Mode::ShowLicense);
    }

    let args = Args {
        // key: matches.get_flag("key"),
    };

    Ok(Mode::Run(args))
}
