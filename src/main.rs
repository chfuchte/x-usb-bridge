mod audio;
mod cli;
mod pipe;
mod web;
mod x32;

use anyhow::{Ok, Result};

use crate::{cli::Mode, pipe::run_pipe, web::run_web_server};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_ansi(true)
        .with_max_level(tracing::Level::DEBUG)
        .init();

    match cli::parse()? {
        Mode::ShowLicense => {
            print_license();

            Ok(())
        }
        Mode::ShowVersion => {
            print_version();

            Ok(())
        }
        Mode::Serve(args) => {
            run_web_server(args.port()).await?;

            Ok(())
        }
        Mode::Pipe(args) => {
            run_pipe(args.timeout())?;

            Ok(())
        }
    }
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
