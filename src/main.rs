mod audio;
mod cli;
mod web;
mod x32;

use anyhow::{Ok, Result};

use crate::cli::Mode;

fn main() -> Result<()> {
    tracing_subscriber::fmt().finish();

    let _args = match cli::parse()? {
        Mode::ShowLicense => {
            print_license();
            return Ok(());
        }
        Mode::ShowVersion => {
            print_version();
            return Ok(());
        }
        Mode::Run(args) => args,
    };

    Ok(())
}

fn print_license() {
    println!(include_str!("../LICENSE.txt"));
}

fn print_version() {
    static CRATE_NAME: &str = env!("CARGO_PKG_NAME");
    static CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");

    match (
        option_env!("BUILD_COMMIT_ID"),
        option_env!("BUILD_COMMIT_DATE"),
    ) {
        (Some(id), Some(date)) => {
            println!("{} {} ({} {})", CRATE_NAME, CRATE_VERSION, id, date);
        }
        _ => {
            println!("{} {}", CRATE_NAME, CRATE_VERSION);
        }
    }
}
