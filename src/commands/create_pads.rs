use crate::otp::pad::PadGenerator;
use anyhow::Result;
use clap::Args;

#[derive(Args, Debug)]
#[command()]
pub struct Cli {}

impl Cli {
    pub fn create_pads(&self) -> Result<()> {
        let mut generator = PadGenerator::new();

        let pad = generator.generate_pad(20);

        println!("Generated pad with id: {}", pad.get_id());
        Ok(())
    }
}
