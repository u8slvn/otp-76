mod commands;
mod otp;
mod parsers;

use anyhow::Result;
use clap::Parser;

use crate::commands::cli::Cli;

fn main() -> Result<()> {
    let cli = Cli::parse();

    cli.exec()
}
