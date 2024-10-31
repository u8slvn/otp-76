use crate::utils::RandomKeyGenerator;
use anyhow::Result;
use clap::Args;

#[derive(Args, Debug)]
#[command()]
pub struct Cli {}

impl Cli {
    pub fn generate_keypad(&self) -> Result<()> {
        let mut generator = RandomKeyGenerator::new();

        let numbers: Vec<u32> = (0..20).map(|_| generator.get_random_key()).collect();

        println!("{:?}", numbers);
        Ok(())
    }
}
