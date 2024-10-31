use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, bin_name = "otp76", disable_help_subcommand = true)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Exec(super::exec::Cli),
    GenerateKeypad(super::generate_keypad::Cli),
}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        match &self.command {
            Commands::Exec(cli) => cli.exec(),
            Commands::GenerateKeypad(cli) => cli.generate_keypad(),
        }
    }
}
