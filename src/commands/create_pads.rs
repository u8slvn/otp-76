use crate::otp;
use crate::parsers::parse_int_arg;
use anyhow::Result;
use clap::Args;

#[derive(Args, Debug)]
#[command()]
pub struct Cli {
    #[arg(long, default_value_t = 10, value_parser = parse_int_arg)]
    nb_pads: u8,
    #[arg(long, default_value_t = 20, value_parser = parse_int_arg)]
    nb_keys: u8,
}

impl Cli {
    pub fn create_pads(&self) -> Result<()> {
        let mut generator = otp::pad::PadGenerator::new();
        let pads = generator.generate_pads(self.nb_pads, self.nb_keys);
        let pad_collection = otp::pad::PadCollection::new(pads);

        println!("{:?}", pad_collection);
        Ok(())
    }
}
