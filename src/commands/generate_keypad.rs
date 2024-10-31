use anyhow::Result;
use clap::Args;


#[derive(Args, Debug)]
#[command()]
pub struct Cli {
}

impl Cli {
    pub fn generate_keypad(&self) -> Result<()> {
        Ok(())
    }
}
